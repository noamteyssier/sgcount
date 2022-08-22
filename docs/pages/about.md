---
layout: page
title: About
permalink: /about/
---

---
## Background
CRISPR screening is a powerful platform in functional genomics to determine which genes affect a quantitative phenotype.
In traditional 1D screens this phenotype takes the form of survival, fluorescence, phagocytosis etc.
Once the cells have been processed and sequenced you will have multiple libraries and sequencing files associated with each of them.

If you are performing a CRISPRi/a v2 screen your sequences will be constructed with the following format:

```
[Constant-Adapter] - [Variable-Region] - [Constant-Adapter]
```

Depending on how you prepared your libraries however, your sequences will have a variable sized constant region, and may be reverse complemented with respect to your library.

---

## Analysis
The ultimate goal of this tool is to count the number of each sgRNA within each sample.

### Steps
1. Variable region placement is identified (this is done on a sample-by-sample basis in case any samples were prepared differently).

2. Library sequences are read in and validated for uniqueness and equivalent size. 
    - This program __will not run__ if there are __duplicate sequences__ (solutions in [usage](/usage/)).
    - This program __will not run__ if the __library sequences are different lengths__.
    - If single mismatches are allowed the unambiguous mismatch library will be generated.

3. Sample variable-region positions are calculated and reverse-complement status is determined.

4. Sequences in each sample are left-trimmed to remove the adapter and right-trimmed to the sequence length of the library.
    - If reverse complement is required the reads are reverse complemented.
    - If single misplace is allowed, positional mismatches are retried with ±1 basepair in the variable-region position.

5. Count tables are written.

---

## Features
- Automatic adapater location identification
- Automatic reverse complement identification
- Single mismatch allowance (one-off in sgRNA Sequence)
- Single misplace allowance (variable region position shift)
- Multiple samples at once
- Multiple cores

---

## Requirements
- sequencing data (`*.fastq`/`*.fastq.gz`)
- reference library (`*.fasta`/`*.fasta.gz`)
- sgRNA-Gene Mapping (`*.txt`) [Optional]

---

## Implementation
If you're interested to see how this works, or want to contribute, feel free to check out the [implementation details](/implementation/).
