[![Crates.io](https://img.shields.io/crates/v/lcov-diff-util.svg)](https://crates.io/crates/lcov-diff-util)
[![Docs.rs](https://docs.rs/lcov/badge.svg)](https://docs.rs/lcov-diff-util/)
[![Travis CI Build Status](https://travis-ci.org/capgelka/lcov-diff.svg?branch=master)](https://travis-ci.org/capgelka/lcov-diff)
![LICENSE](https://img.shields.io/crates/l/lcov-diff.svg)
# Lcov Diff

## Prerequisites

* Rust, install rustup, [see](https://rustup.rs/)
* Cargo
* genhtml(install `lcov` via `apt`)

## Build

```bash
git clone <repo>
cd lcov-diff
cargo build --release
```

See `target/release`
If you would like to build `debug` version, just run: `cargo build`

To run tests:

```
cargo test --all
```

To install (to `$HOME/.cargo/bin`):

```
cargo install --path
```

See `cargo` [docs](https://doc.rust-lang.org/cargo/commands/cargo-doc.html)

## Tool to diff lcov files.

```
USAGE:
    lcov-diff [FLAGS] [OPTIONS] <FILE>...

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information
    -v, --verbose    Show Debug logging

OPTIONS:
    -o, --output <PATH>    output file to write (stdout if not presented)
    -w, --web <web>        Generate html report from output file (default name web)

ARGS:
    <FILE>...    Files to process, right now just two of them
```

The output file contains the only lines/functions/basic blocks which are presented only in the first lcov file.
To generate html report for the diff use `genhtml tool`, [see](#Prerequisites).

```bash
lcov-diff first.lcov second.lcov -o out.lcov
genhtml --ignore-errors source -o web out.lcov
```

Or just run with `-w`/`--web` option to generate it automatically after getting diff file
(if `output` is not set a tmp file will be used for it).
The default output directory name is `web`.
Also, it gets into panic if `genhtml` could not be invoked.

```bash
lcov-diff first.lcov second.lcov -w LCOV_HTML_DIR -o out.lcov
```
