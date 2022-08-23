---
layout: page
title: Implementation
permalink: /implementation/
---

---
## Implementation
Ultimately this is written to be as fast as possible by using sequence hashing.

The library is hashed upon entry, sequences are trimmed to the positions of interest, then checked against the hashmap and counted.

### Reading in the Library
The library is read-in as a `fasta` formatted file into a `HashMap`. 

It is checked for uniqueness and requires each sequence to have an equal length.

If duplicates sequences are found `sgcount` will quit.

### Mismatch Library
To allow for one-offs in the reference library I create an unambiguous one-off hashmap by using the sequences within the library.

In essence, each sequence in the library has every valid 1-hamming-distance sequence generated.

Then any of those generated sequences which could map back to more than one reference sequence is removed.

This front-loads the search by pre-generating all the possible one-offs, and still allows O(1) search when iterating over the sequences.

### Variable-Region Positioning and Reverse-Complement Identification
I used an [entropy](https://en.wikipedia.org/wiki/Entropy_(information_theory))-based approach to determine where the variable region would be in the sequencing dataset.

Because the sequences are constructed with a constant region it means that the nucleotide entropy (i.e. the randomness in observing a nucleotide `A`, `C`, `T`, `G`) in that region will be considerably lower than in the variable region.

As a result, if you were to plot the entropy in each position across the sequence you will see near-zero entropy everywhere except for the variable region.

```
[ 1e-3 | 1e-3 | 2e-3 | 1e-3 | 0.5 | 0.7 | 0.8 | 0.7 | 1e-4 | 1e-3 | 1e-3 | 2e-3 ]
                            ^                       ^
                            |                       |
                    variable_start            variable_end
```

To determine where this region starts and ends, I first calculate the positional entropy in the reference library, then sample a few thousand sequences from the sample libraries, and then determine which positions minimize a minimum-squared-error between the two. 

```
[  0.5 |  0.7 |  0.8 |  0.7 ]                               << Reference Entropy
[ 1e-3 | 2e-3 |  0.5 |  0.7 |  0.8 |  0.7 | 1e-4 | 1e-3 ]   <<  Observed Entropy
```

```
[ 1e-3 | 2e-3 |  0.5 |  0.7 |  0.8 |  0.7 | 1e-4 | 1e-3 ]
[ 0.5  |  0.7 |  0.8 |  0.7 ]
       [ 0.5  |  0.7 |  0.8 |  0.7 ]
              [  0.5 |  0.7 |  0.8 |  0.7 ]                 << Minimum MSE Position
                     [  0.5 |  0.7 |  0.8 |  0.7 ]
                            [  0.5 |  0.7 |  0.8 |  0.7 ]
```


This strategy easily lets you determine the reverse complement as the positional entropy vector can just be flipped and searched in the same manner.

### Single-Position Misplacement
To allow for indels before the variable region I've implemented a single-position misplacement allowment.

Visually this looks like this:
```
ACTGGACAA|VARIABLE|
ACTGGACAA|VARIABLE|
ACTGGACAA|VARIABLE|
ACTGGACA|VARIABLE|     << Single deletion
ACTGGACAAA|VARIABLE|   << Single insertion
ACTGGAC|VARIABLE|      << Double deletion (not supported)
ACTGGACAAAA|VARIABLE|  << Double insertion (not supported)

```

Since the reads are trimmed to the expected variable length before searching the hashmap, these last two examples would've been missed even though their variable sequence occured without any mismatches.

To capture these sequences the reads are first tried at the expected variable position, but if there are no matches they are retried with a `-1` variable position, and again with a `+1` variable position.

If both of these fail then it is considered a true failure and will not be captured by `sgcount`.

This could be repeated _ad infinitum_ but I think it starts to lose biological credibility as the indel size increases.

In my experience this has recovered an extra 3-5% of sequences, which is not insignificant, and could make a very large difference if those sgRNAs were low abundance to begin with.
