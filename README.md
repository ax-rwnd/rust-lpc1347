# rust-lpc1347
This repository aims to provide a rust crate for interfacing with the LPC1347 and to serve as a roadmap for preparing your programs to run on NXP MCUs. It has been generated using `svd2rust` and `rustfmt` according to the instructions given at [docs.rs](https://docs.rs/svd2rust/0.11.4/svd2rust/).

An issue that is resolved manually in the package is that NXP truncates long `<name>` tags, this seems to be common for most of their SVDs and is not catched by `svd2rust`. If you are appropriating some other SVD into a rust crate you may want to use `dupes.pl` or some other tool to check the SVD for duplicated and truncated names before attempting to convert it.

With the introduction of version `0.13` of svd2rust the support of overlapping registers is introduced. This requires the use of `--nightly` flag.

## Building

```
svd2rust --nightly -i LPC13Uxx_patched.svd

rm -rf src

form -i lib.rs -o src/

rm lib.rs

cargo fmt
```

## Usage
Add `rust-lpc1347` as a dependency to your Cargo.toml file:
```toml
    [dependencies.lpc1347]
    features = ["rt"]
    version = "0.2.1" # replace this with the current version...
    git = "https://github.com/ax-rwnd/rust-lpc1347.git"
```

## Changelog

### [0.2.1] - 2018-07-17
- Updates svd2rust to `0.13.1`. svd2rust uses the `--nightly` feature. Tested with
  `rustc 1.28.0-nightly (a805a2a5e 2018-06-10)` Which is working with cortex-m `0.5.x`.

### [0.2.0] - 2018-05-23
- Updates svd2rust to `0.13` in order to work with newer versions of RTFM and cortex-m, tested with
  `rustc 1.28.0-nightly (71e87be38 2018-05-22)`

### [0.1.5] - 2018-04-09
- Updates the SVD to work with newer versions of RTFM and rust, tested with
  `rustc 1.27.0-nightly (056f589fb 2018-04-07`
- Adds a manual patch for overlapping USART registers
