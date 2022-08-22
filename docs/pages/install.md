---
layout: page
title: Install
permalink: /install/
---

---
## Installing from crates.io
sgcount can be installed directly from [crates.io](https://crates.io/crates/sgcount) using the `cargo` package manager.

If you've used rust executables before and already have cargo installed you can skip the following step and go directly to installing with `cargo`.

#### Installing `cargo`
If you've never used rust you'll need to install cargo.
Here is a oneliner from [rustup](https://rustup.rs) which handles the installation for you.
```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

#### Installing `sgcount` with `cargo`
Once you have `cargo` you can install `sgcount` directly from [crates.io](https://crates.io/crates/sgcount).
```bash
cargo install sgcount
```

---
## Building from source
If you would rather build from source instead you can clone the repo and then install using `cargo`.
```bash
git clone https://github.com/noamteyssier/sgcount
cd sgcount
cargo install --path .
```
