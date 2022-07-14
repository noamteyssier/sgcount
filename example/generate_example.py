import numpy as np

ALPHABET = ['A', 'C', 'G', 'T']
QUAL = ['1', '2', '3', '4', '5']

LIB_NUM = 100
LIB_SIZE = 20

TEST_NUM = 1000
TEST_SIZE = 80
TEST_OFFSET = 0

def generate_library():
    for idx in np.arange(LIB_NUM):
        header = f"lib.{idx}"
        seq = ''.join([str(x) for x in np.random.choice(ALPHABET, size=LIB_SIZE)])
        yield header, seq

def generate_sequences(library):
    for idx in np.arange(TEST_NUM):
        lib_choice = ''.join(np.random.choice(library, size=1))
        header = f"seq.{lib_choice}.{idx}"
        prefix = ''.join([str(x) for x in np.random.choice(ALPHABET, size=TEST_OFFSET)])
        suffix = ''.join([str(x) for x in np.random.choice(ALPHABET, size=TEST_SIZE - TEST_OFFSET - LIB_SIZE)])
        seq = prefix + lib_choice + suffix
        qual = ''.join([str(x) for x in np.random.choice(QUAL, size=TEST_SIZE)])
        yield header, seq, qual, lib_choice

library_dict = {}
with open("example/library.fa", "w+") as f:
    for h, s in generate_library():
        library_dict[s] = 0
        f.write(f">{h}\n{s}\n")

with open("example/sequence.fq", "w+") as f:
    for h, s, q, l in generate_sequences(list(library_dict.keys())):
        library_dict[l] += 1
        s = ''.join([x if idx != 2 else "N" for idx, x in enumerate(s)])
        f.write(f"@{h}\n{s}\n+\n{q}\n")

with open("example/counts.txt", "w+") as f:
    for k in library_dict.keys():
        v = library_dict[k]
        f.write(f"{k}\t{v}\n")
