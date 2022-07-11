use fxread::{Record, FastxRead};
use anyhow::Result;

pub struct Trimmer {
    reader: Box<dyn FastxRead<Item = Record>>,
    offset: usize,
    size: usize
}
impl Trimmer {
    pub fn from_reader(
            reader: Box<dyn FastxRead<Item = Record>>,
            offset: usize,
            size: usize) -> Self {
        Self { 
            reader , 
            offset, 
            size 
        }
    }

    fn trim_sequence(&self, token: &str) -> Result<String> {
        match token.len() >= self.offset + self.size {
            true => {
                Ok ( 
                    token
                        .chars()
                        .skip(self.offset)
                        .take(self.size)
                        .collect()
                    )
            },
            false => Err(anyhow::anyhow!("Provided sequence {} is not large enough", token))
        }
    }

    fn trim_record(&self, record: Record) -> Result<Record> {
        let mut trim = Record::new();
        trim.set_id(record.id().to_owned());
        trim.set_seq(self.trim_sequence(record.seq())?);
        Ok(trim)
    }
}

impl FastxRead for Trimmer {
    fn next_record(&mut self) -> Result<Option<Record>> {
        match self.reader.next_record()? {
            Some(r) => Ok(Some(self.trim_record(r)?)),
            None => Ok(None)
        }
    }
}

impl Iterator for Trimmer {

    type Item = Record;

    fn next(&mut self) -> Option<Self::Item> {
        match self.next_record() {
            Ok(r) => r,
            Err(_) => panic!("Unexpected EOF")
        }
    }

}

