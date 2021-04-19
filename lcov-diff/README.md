[![Crates.io](https://img.shields.io/crates/v/lcov-diff.svg)](https://crates.io/crates/lcov-diff)
[![Docs.rs](https://docs.rs/lcov/badge.svg)](https://docs.rs/lcov-diff/)
[![Travis CI Build Status](https://travis-ci.org/capgelka/lcov-diff.svg?branch=master)](https://travis-ci.org/capgelka/lcov-diff)
![LICENSE](https://img.shields.io/crates/l/lcov-diff.svg)
# Lcov Diff library

This is a library to create delta of 2 lcov reports which is used by `lcov-diff-util` crate.

It is implemented as extension trait `Diff` for `lcov::Report` [`lcov`](https://crates.io/crates/lcov) crait. A new method `diff` was added for `lcov::Report` and it's internal substructures.
