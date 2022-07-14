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
            Err(why) => panic!("{}", why)
        }
    }

}

#[cfg(test)]
mod test {

    use fxread::{FastaReader, FastxRead, Record};
    use super::Trimmer;

    fn reader() -> Box<dyn FastxRead<Item = Record>> {
        let sequence: &'static [u8] = b">seq.0\nACTG\n";
        Box::new(FastaReader::new(sequence))
    }

    #[test]
    fn build() {
        let trimmer = Trimmer::from_reader(reader(), 0, 2);
        assert_eq!(trimmer.into_iter().count(), 1);
    }

    #[test]
    fn trim_no_offset() {
        let mut trimmer = Trimmer::from_reader(reader(), 0, 2);
        let record = trimmer.next().unwrap();
        assert_eq!(record.seq(), "AC");
    }

    #[test]
    fn trim_with_offset() {
        let mut trimmer = Trimmer::from_reader(reader(), 1, 2);
        let record = trimmer.next().unwrap();
        assert_eq!(record.seq(), "CT");
    }

    #[test]
    #[should_panic]
    fn trim_with_oversize_offset() {
        let mut trimmer = Trimmer::from_reader(reader(), 4, 2);
        trimmer.next().unwrap();
    }
    #[test]
    #[should_panic]
    fn trim_with_oversize_size() {
        let mut trimmer = Trimmer::from_reader(reader(), 0, 5);
        trimmer.next().unwrap();
    }
}
