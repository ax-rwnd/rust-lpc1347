# rust-lpc1347
This repository aims to provide a rust crate for interfacing with the LPC1347 and to serve as a roadmap for preparing your programs to run on NXP MCUs. It has been generated using `svd2rust` and `rustfmt` according to the instructions given at [docs.rs](https://docs.rs/svd2rust/0.11.4/svd2rust/).

An issue that is resolved manually in the package is that NXP truncates long `<name>` tags, this seems to be common for most of their SVDs and is not catched by `svd2rust`. If you are appropriating some other SVD into a rust crate you may want to use `dupes.pl` or some other tool to check the SVD for duplicated and truncated names before attempting to convert it.

## Usage
Add `rust-lpc1347` as a dependency to your Cargo.toml file:
```toml
    [dependencies.lpc1347]
    features = ["rt"]
    version = "0.1.5" # replace this with the current version...
    git = "https://github.com/ax-rwnd/rust-lpc1347.git"
```

## Changelog

### [0.1.5] - 2018-09-04
- Updates the SVD to work with newer versions of RTFM and rust, tested with
  `rustc 1.27.0-nightly (056f589fb 2018-04-07`
- Adds a manual patch for overlapping USART registers
