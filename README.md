[![](https://github.com/regiontog/sykl/workflows/Rust/badge.svg)](https://github.com/regiontog/sykl/actions)
# Components
## Sykl
https://github.com/regiontog/sykl/blob/master/components/sykl

## Sykl-rest
https://github.com/regiontog/sykl/blob/master/components/sykl-rest

## Bikeshare library
https://github.com/regiontog/sykl/blob/master/components/bikeshare

## Sykl formatting library
https://github.com/regiontog/sykl/blob/master/components/sykl-formatting

# Rust toolchain
To run `cargo` commands for generating documentation, compiling or testing you need to have a [rust toolchain](https://rustup.rs/) installed.

# Documentation
This repo hosts documentation on Github pages since docs.rs only has documentation for libraries, but untill a [bug](https://github.community/t5/GitHub-Actions/Github-action-not-triggering-gh-pages-upon-push/td-p/26869) in github is resolved it is not visible as pushes from GitHub actions don't trigger the Github Pages build. The documentation can be build from source in the source directory with:
```bash
> cargo doc --open --no-deps
```

# Testing
You can run the tests(requires nightly) by using the command:
```bash
> cargo +nightly test
```
