![logo](https://raw.githubusercontent.com/noamteyssier/sgcount/gh-pages/images/logo.svg)

# sgcount

[![MIT licensed](https://img.shields.io/badge/license-MIT-blue.svg)](./LICENSE.md)
![actions status](https://github.com/noamteyssier/sgcount/workflows/CI/badge.svg)
[![codecov](https://codecov.io/github/noamteyssier/sgcount/branch/main/graph/badge.svg?token=IM36KMKJ9T)](https://codecov.io/github/noamteyssier/sgcount)

`sgcount` is a fast and flexible sgRNA counter for CRISPR screens.

This was developed to be a simple yet powerful tool to take raw sequencing files to count tables.
This will be the preprocessing step for [CRISPRBrain](https://crisprbrain.org/) and will eventually be available to perform analysis directly from the browser - but is available here as a standalone tool for those who would prefer local analysis. 

## [About](https://noamteyssier.github.io/sgcount/about)
`sgcount` was designed to be a single, easy, and fast interface for researchers who just want to get the results of their CRISPR screen.

To learn more about the features of this tool over existing methods check out the [about](https://noamteyssier.github.io/sgcount/about/) page.

## [Install](https://noamteyssier.github.io/sgcount/install)
No wasted time fiddling with conda environments - `sgcount` is written in rust and is backed by its powerful package manager `cargo`.

```bash
cargo install sgcount
```

One line installation can be found in the [install](https://noamteyssier.github.io/sgcount/install) instructions.

## [Usage](https://noamteyssier.github.io/sgcount/usage)
If you're ready to go an run your first screen check out the [usage](https://noamteyssier.github.io/sgcount/usage).

```bash
sgcount -l <library> -i <sample>
```

## Running differential expression and gene aggregation

Once you have your counts - check out my tool [`crispr_screen`](https://noamteyssier.github.io/crispr_screen/)
to perform the differential expression and gene-level aggregation.

# External Links
* [Github Repo](https://github.com/noamteyssier/sgcount)
* [Crates](https://crates.io/crates/sgcount)
* [Docs](https://docs.rs/sgcount)
