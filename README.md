![logo](https://raw.githubusercontent.com/noamteyssier/sgcount/gh-pages/images/logo.svg)

# sgcount
`sgcount` is a fast and flexible sgRNA counter for CRISPR screens.

This was developed to be a simple yet powerful tool to take raw sequencing files to count tables.
This will be the preprocessing step for [CRISPRBrain](https://crisprbrain.org/) and will eventually be available to perform analysis directly from the browser - but is available here as a standalone tool for those who would prefer local analysis. 

## [About](/pages/about.md)
`sgcount` was designed to be a single, easy, and fast interface for researchers who just want to get the results of their CRISPR screen.

To learn more about the features of this tool over existing methods check out the [about](/pages/about.md) page.

## [Install](/pages/install.md)
No wasted time fiddling with conda environments - `sgcount` is written in rust and is backed by its powerful package manager `cargo`.

```bash
cargo install sgcount
```

One line installation can be found in the [install](/pages/install.md) instructions. 

## [Usage](/pages/usage.md)
If you're ready to go an run your first screen check out the [usage](/pages/usage.md).

```bash
sgcount -l <library> -i <sample>
```

# External Links
* [Github Repo](https://github.com/noamteyssier/sgcount)
* [Crates](https://crates.io/crates/sgcount)
* [Docs](https://docs.rs/sgcount)
