# Installation
This can be installed from the commandline using cargo.
```bash

# installing from crates.io
cargo install sgcount

# installing from github
git clone https://github.com/noamteyssier/sgcount
cd sgcount
cargo install --path .
```

# Usage
This is meant to be used as a commandline tool and it expects at 
minimum two files.

The first is the `Library`, which is a fasta formatted file describing
the sequencing library that the sequences should be aligned to.

The second is the `Sequencing`, which is generally a fastq or fastq.gz
file representing the sequencing results of a CRISPRi/a screen. 

## Running sgcount

### Basic Usage
The experiment can then be run using the commandline interface.

```bash
sgcount -l <path_to_library> -i <path_to_sequencing>
```

### Multiple Sequencing Files
If you have multiple files to count you can provide them as
extra arguments to the `-i` flag.

```bash
sgcount -l <path_to_library> -i <path_to_sequencing_a> <path_to_sequencing_b>
```

### Setting the Offset
The program will automatically determine the offset if this flag is
not provided.
However, if you know the offset a priori you can supply it with the
`-n` flag.
If you recover all zeros in your counts it is recommended to not 
supply this flag and let the program determine the offset algorithmically.

For example, if you have a 10bp adapter sequence before the sgRNA
then you can run the following:
```bash
sgcount -l <path_to_library> -i <path_to_sequencing> -n 10
```

### Allowing Mismatches
By default sgcount will only allow for exact matches, but if you would
like to allow a single mismatch you can specify it with the `-m` flag.

```bash
sgcount -l <path_to_library> -i <path_to_sequencing> -m
```

### Setting the Output File
By default sgcount will write the results to stdout, but if you'd like to
specify the output path directly you can do so with the `-o` flag.

```bash
sgcount -l <path_to_library> -i <path_to_sequencing> -o <path_to_output>
```


## Example Fasta Library
An example library is provided in `example/library.fa`


```text
>lib.0
GGACAGGTCCGGTGTACGTC
>lib.1
TAGATGTGCCAACCCGGTCA
>lib.2
AAATTGCGATCTCACACCTT
>lib.3
CGAAAACGAGTGGGACACAG
>lib.4
ATACTGCGGCGGAGGCGAGA
>lib.5
TAGGAAAATTTCGCGACCTT
>lib.6
ACGGCGGACGCGACGAGGGC
>lib.7
ACTGAACCTCCCCGTCAGAT
>lib.8
GAGACCCGAATTGAGATTCG
>lib.9
GGGGGGGTGAACGGGGCATG
>lib.10
TCCCAATAGGATCGAGGGAT
```

## Example Sequencing
An example of some sequencing data is provided in `example/sequence.fq`

```text
@seq.ACGGCGGACGCGACGAGGGC.0
ACNGCGGACGCGACGAGGGCCGTTACCGCTAAGTACGTACTCACCCTTAAAATCCTCTAGTGGGGCTTCGCCGTGACTGG
+
52512132223325121113311131433113221533223312544523323525432144232354555542453331
@seq.AAAATCCTTGTAACACTTAA.1
AANATCCTTGTAACACTTAAATGAAGAGCAACAGGAAGGACAGAACGGTACAGTTGGCAGTGTAGTCTTATAACAAAAGC
+
33443352535454311153551315134233351422123355551544334532355212432451341232324451
@seq.CTGGATAGATACTTGCTCCC.2
CTNGATAGATACTTGCTCCCTCATTAAAATCGGCCGATTGCACTTCTAGCATTTACACATCATGTTCAGCGGTTCCTTCC
+
51125523541135212515355332411353521352343522114335225313435354411542221325454553
@seq.TAGATGTGCCAACCCGGTCA.3
TANATGTGCCAACCCGGTCAGGGCTAGGGTAATGTTGCTCACGAAGGACGCTGTATCGGTAACTTTCGTGAGTTTTATAA
+
14155145244353554252432314214523345343345343111345514445213215535411424522443233
@seq.GGCTTCATCCCTTCCGTCGC.4
GGNTTCATCCCTTCCGTCGCGGAAGAAGACCTATACCGCGTCATACTTAGTGAGACTCCCTCTGACTCAGCTAAGGAACT
+
21141524354243332144232121225532514212133153513145533521123122445333425212234133
@seq.GTCCGACTTAACTCTGATCG.5
GTNCGACTTAACTCTGATCGCTACCTCTGTCTTGCGCAATTATAGCTGCAAAATCCCCGAACCATGAACCGGTTCAACTA
+
12452351113143455235111211443551441153432225551215453143232551512314315512131524
@seq.GTACCGAAGGCATGACCGGA.6
GTNCCGAAGGCATGACCGGAATACTCTCTCATGAACAGCTTCTATACGGCTGAACAGTCAAACGATTCGAGATTTAAAAG
+
21324412223434123235125112154152325132134523153121341155432324125231155323412131
@seq.CCTTAAGACGCAGAAGACAA.7
CCNTAAGACGCAGAAGACAAGTTTTTATGATATTTGCCAGGTCACAATTGCAGTAGCCGATCGCATTTACGCCTAAAGTG
+
25141424511335251544334452434232254554323151242121223125153241543151325332452544
@seq.CAAGCCCGCTTGAGCAAAAC.8
CANGCCCGCTTGAGCAAAACCACCCCCAGAGAGATCGGAGAGATGCTGGAATCTCTTCTATTGTAGAAACATAACCATGA
+
34541212232233243232214513432152254123243411455252552313213521521412321335553325
@seq.CATAACGAGTCACTCTGTGA.9
CANAACGAGTCACTCTGTGAGTAGTACCTATGCTTCCCTCGCCAATAAGCTATGTATAAAGGGGTTATAGCTCGCGAAAG
+
21432214421423322312445125254412151334241511553241452322215523213234532544341141
@seq.TCGAAGAGCACAACGATCGA.10
TCNAAGAGCACAACGATCGATCAACACGTCTGGTGTTGTATAGAGTCTAGATCGGCCGTTCGAGGCTTAACTAGGCGTTA
+
31524543552542135543314525313343354434234235311435152211153222212542532154313425
```
