---
layout: page
title: Requirements
permalink: /requirements/
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

### Reference Library
This should be a `FASTA` formatted file, and can have any file extension (`*.fasta`, `*.fa`) and their gzip counterparts.

This is expected to the the __variable__ regions of your sgRNAs - and should not have any adapter regions on either side.

Here is tiny example library:

```
>lib.0
ATAGCCCGGCGGTCTGCTGG
>lib.1
TAAGGCACTATAGCAATGAG
>lib.2
GTAGATAAAACGTGTGGCCC
>lib.3
AAGGCGACCATCTACCCTTG
```

### Sample Library
These are the larger `FASTQ` formatted files which represent the results of your screen. 
They can have any file extension (`*.fastq` / `*.fq` ) and their gzip counterparts.

This are __not expected to be the variable regions__, so don't worry about pretrimming these sequences before running them with `sgcount`.

Here is a tiny example sample:

```
@seq.AACGTTCTCCAGTATGAAAG.0
ATNGCAACGTTCTCCAGTATGAAAGTAGCGACAAGACGGGCCAAGAGGGACTGCGCACCACGTAGTTACCCCGATCCTAT
+
43212322322242515413324331541432414553224213511111344532442224113253532413451225
@seq.CGGTTCCCTGCCGCTACGAG.1
ATNGCCGGTTCCCTGCCGCTACGAGTAGCGACAAGACGGGCCAAGAGGGACTGCGCACCACGTAGTTACCCCGATCCTAT
+
23233555242215242532355415123114534342422111212445152424453152255425331534444213
@seq.CTCGCCGCGCGGCACTATTG.2
ATNGCCTCGCCGCGCGGCACTATTGTAGCGACAAGACGGGCCAAGAGGGACTGCGCACCACGTAGTTACCCCGATCCTAT
+
54532443112431133412311213532322244241224451345215242125451241523232121145343513
@seq.TATAGACATATTATACGTCC.3
ATNGCTATAGACATATTATACGTCCTAGCGACAAGACGGGCCAAGAGGGACTGCGCACCACGTAGTTACCCCGATCCTAT
+
33231435244335232142144245314521453354531535215154523311555133141253412544112225
@seq.GGTTTGTTACGCGAGCAGTT.4
ATNGCGGTTTGTTACGCGAGCAGTTTAGCGACAAGACGGGCCAAGAGGGACTGCGCACCACGTAGTTACCCCGATCCTAT
+
52245315235112214142511531543122452153335313154325215245554114252235434421423233
```

### sgRNA to Gene Mapping
Generally, and especially for v2 libraries, you have multiple sgRNAs for each target gene. 

If you'd like to include the sgRNA &rarr; Gene information in the final output table (like if you were planning on doing differential expression next!) you can include that information with a two-column tab-delim file.

This `<s2g>` is a two-column tab-delim file where the first column is the sgRNA name as it appears in your `<input_library>` and the second column is the gene that it targets (or any alias you would like to assign it).

```
sgrna_1	AP2S1
sgrna_2	AP2S1
sgrna_3	RFX3
sgrna_4	RFX3
sgrna_5	LDB1
sgrna_6	LDB1
sgrna_7	LDB1
```

If you don't have this table, and you don't know how to make it easily with a bash script, I invite you to checkout my tool [`fxtools`](https://github.com/noamteyssier/fxtools#sgrna-table) where I've written in a simple tool to extract the gene names from sgRNA libraries.

