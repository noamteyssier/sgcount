---
layout: page
title: Usage
permalink: /usage/
---

---
* TOC
{:toc}

---
## Requirements
Before starting you'll need to make sure you have the following files prepared: 

- Reference library ( `*.fasta` / `*.fasta.gz` )
- Sample Libraries ( `*.fastq` / `*.fastq.gz` )
- sgRNA to Gene Mapping ( `*.txt` / `*.tsv` / `*.tab` ) [ _optional_ ]

If you already know what these are, feel free to move on, otherwise take a closer look at the [requirements](requirements.md).


---
## Counting your samples

### Basic usage
This is meant to be run from the commandline, and follows a simple interface. 

You specify your library with `-l`, your sample(s) with `-i`, and your output file with `-o`.

```bash
sgcount \
  -l <input_library> \
  -i <sample_a> <sample_b> <sample_n> \
  -o <output_file>
```

This will create a tab-delim table in your `<output_file>`.

In this case there will be `N+1` columns in the file, where `N` is the number of samples you provided with the `-i` flag.
The first column will be the sgRNA, and the remaining columns will show the number of counts of that sgRNA in the provided sample.

You'll see that the remaining columns are named `sample_1`, `sample_2`, ... `sample_n` by default, which map to the ordering which they were provided in the argument.

---

### Counting your samples with aliased names
Sometimes sequencing libraries have terribly informative names like `SetA_9991_10215_81562_HJJGLBGX7_twb3-16_BOT25_AGTTCC_R1.fastq.gz` but you might just want to refer to it as `BOT25`. 

To provide custom library names to your samples you can use the `-n` flag.


```bash
sgcount \
  -l <input_library> \
  -i <sample_a> <sample_b> <sample_n> \
  -n name_1 name_b name
  -o <output_file>
```

Keep in mind that this is a one-to-one mapping, so in this case `<sample_a>` will have the column name `name_1`, `<sample_b>` will have the column name `name_b`, and so on.

---
### Including sgRNA &rarr; gene mapping
Generally, especially for v2 libraries, you have multiple sgRNAs for each target gene. 
You can specify that relationship by providing a sgRNA-gene mapping to the `-g` flag.

This `<s2g>` is a two-column tab-delim file where the first column is the sgRNA name as it appears in your `<input_library>` and the second column is the gene that it targets (or any alias you would like to assign it).

```bash
sgcount \
  -l <input_library> \
  -i <sample_a> <sample_b> <sample_n> \
  -g <s2g> \
  -o <output_file>
```

You'll see in this case there will be `N+2` columns in the output file, where `N` is the number of samples you provided with the `-i` flag.
The first column will be the sgRNA, the second the associated gene, and the remaining will show the number of counts of that sgRNA in the provided sample.

---
### Running on multiple cores
Each sample can be run on a separate thread for large speedups. 

Currently this is done at the sample level, so you will not see an improvement for using more threads than samples provided (though this will maybe change in the future).

You can specify the number of threads  with the `-t` flag.

```bash
sgcount \
  -l <input_library> \
  -i <sample_a> <sample_b> <sample_n> \
  -o <output_file> \
  -t 3
```


---
## Advanced Usage

### Removing Mismatches
`sgcount` will create an unambiguous one-off mismatch library to match your sequences against by default (see [features](/about.md#features)), but if you would rather only accept exact matches you can specify that with the `-x` flag.

```bash
sgcount \
  -l <input_library> \
  -i <sample_a> <sample_b> <sample_n> \
  -o <output_file> \
  -x
```

---
### Removing Positional Misplacements
`sgcount` will attempt to match your sequences with ±1 basepair shifts from the variable-region if the sequence wasn't found on its first pass.

If you would prefer to remove this behavior you can do so with the `-p` flag.

```bash
sgcount \
  -l <input_library> \
  -i <sample_a> <sample_b> <sample_n> \
  -o <output_file> \
  -p
```

---
### Providing an adapter length
`sgcount` will determine where the variable region is in your reads automatically (see [features](/about.md#features) and [implementation](/implementation.md#variable-region-positioning-and-reverse-complement-identification)).

However, if it is unable to find it, and you are __certain__ of its location in your sequences, you can specify your adapter length with the `-a` flag.

```bash
sgcount \
  -l <input_library> \
  -i <sample_a> <sample_b> <sample_n> \
  -o <output_file> \
  -a 10
```

---
### Fixing reverse-complement status.
`sgcount` will determine whether the reads are reverse-complemented with respect to the library (see [features](/about.md#features) and [implementation](/implementation.md#variable-region-positioning-and-reverse-complement-identification)).

However, if it can't identify the reverse-complement status of your sequences, and you are __certain__ of its status, you can specify that they are reverse complemented with the `-r` flag.

```bash
sgcount \
  -l <input_library> \
  -i <sample_a> <sample_b> <sample_n> \
  -o <output_file> \
  -r
```
---
