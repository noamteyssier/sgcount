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

### Providing Sample Names
If you have a shorthand alias for your sample names you can provide
them with the `-n` flag.
Note that the number of sample names must be equal to the number of
provided samples (the program will quit otherwise).

```bash
sgcount -l <path_to_library> -i <path_to_sequencing_a> <path_to_sequencing_b> -n <name_a> <name_b>
```

### Providing a Gene -> sgRNA Mapping
If you would like to associate each sgRNA with its parent gene you can
provide that mapping with the `-g` flag.
This will expect a two column tab-delim file.
The first column will be the gene, and the second will be the sgRNA name (not sequence).
This will be validated against the library - and any sgRNAs that cannot be mapped against
this table in the library will cause the program to quit. 

If your library is fasta formatted and the gene is provided in the header you can generate
this file easily using my tool `sgrna-table` within [fxtools](https://github.com/noamteyssier/fxtools).

```bash
sgcount -l <path_to_library> -i <path_to_sequencing> -g g2s.txt
```

### Setting the Offset
The program will automatically determine the offset if this flag is
not provided.
However, if you know the offset a priori you can supply it with the
`-a` flag.
If you recover all zeros in your counts it is recommended to not 
supply this flag and let the program determine the offset algorithmically.

For example, if you have a 10bp adapter sequence before the sgRNA
then you can run the following:
```bash
sgcount -l <path_to_library> -i <path_to_sequencing> -a 10
```

### Disallowing Mismatches
By default sgcount will allow single mismatches, but if you would
like to have only exact matches you can specify it with the `-x` flag.

```bash
sgcount -l <path_to_library> -i <path_to_sequencing> -x
```

### Setting the Output File
By default sgcount will write the results to stdout, but if you'd like to
specify the output path directly you can do so with the `-o` flag.

```bash
sgcount -l <path_to_library> -i <path_to_sequencing> -o <path_to_output>
```

### Shutting off verbosity
By default the program will show how much time is being elapsed as well as
update the user on its progress. You can shut off this behavior with the
`-q` flag.

```bash
sgcount -l <path_to_library> -i <path_to_sequencing> -q
```


## Example Fasta Library
An example library is provided in `example/library.fa`


```text
>lib.0
ATAGCCCGGCGGTCTGCTGG
>lib.1
TAAGGCACTATAGCAATGAG
>lib.2
GTAGATAAAACGTGTGGCCC
>lib.3
TTCATACAATAACGACGTGC
>lib.4
AAGGCGACCATCTACCCTTG
>lib.5
CGCATAAACCCTTTCAACTG
>lib.6
GGAGTGGAGCGCTGAGTCGT
>lib.7
GGTAAGTACACATCGCCATG
>lib.8
CAGGTAGGACTACAGAGCTG
>lib.9
GCCTATGGTTGGTAGGCAAG
>lib.10
CGGGGCGTGCTATACGCATT
```

## Example Sequencing
An example of some sequencing data is provided in `example/sequence.fq`

```text
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
@seq.ATACGCATCTTCGGAATGTA.5
ATNGCATACGCATCTTCGGAATGTATAGCGACAAGACGGGCCAAGAGGGACTGCGCACCACGTAGTTACCCCGATCCTAT
+
31143423343255242141513351253142515145434443123244145415354115445255254212451244
@seq.AGGGTGCTTTTGATGTGGAT.6
ATNGCAGGGTGCTTTTGATGTGGATTAGCGACAAGACGGGCCAAGAGGGACTGCGCACCACGTAGTTACCCCGATCCTAT
+
53251513225425232352211233534114522215223524153354423322333521454544324423154421
@seq.CGCTCGCCTTCAAGCTACAT.7
ATNGCCGCTCGCCTTCAAGCTACATTAGCGACAAGACGGGCCAAGAGGGACTGCGCACCACGTAGTTACCCCGATCCTAT
+
44232545355144343215132235132555415544524212153151242432352221425555451214415433
@seq.ATCCGTTAACACCCGTGTAA.8
ATNGCATCCGTTAACACCCGTGTAATAGCGACAAGACGGGCCAAGAGGGACTGCGCACCACGTAGTTACCCCGATCCTAT
+
11525225352243452555523453253222354324253121311522125114552224532131353125523242
@seq.TTTTCGAGATATCTTGCCTT.9
ATNGCTTTTCGAGATATCTTGCCTTTAGCGACAAGACGGGCCAAGAGGGACTGCGCACCACGTAGTTACCCCGATCCTAT
+
15352412434253544231322442452233153353522434245343321511215322155333313541233112
@seq.AACGTTCTCCAGTATGAAAG.10
ATNGCAACGTTCTCCAGTATGAAAGTAGCGACAAGACGGGCCAAGAGGGACTGCGCACCACGTAGTTACCCCGATCCTAT
+
24352514151524243135221555342112334153424555141234231424555513545151511254444444
```
