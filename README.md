# battop

[![Latest Version](https://img.shields.io/crates/v/battop.svg)](https://crates.io/crates/battop)
[![Build Status](https://travis-ci.org/svartalf/rust-battop.svg?branch=master)](https://travis-ci.org/svartalf/rust-battop)
[![dependency status](https://deps.rs/crate/battop/0.3.0/status.svg)](https://deps.rs/crate/battop/0.3.0)
![Apache 2.0 OR MIT licensed](https://img.shields.io/badge/license-Apache2.0%2FMIT-blue.svg)

`battop` is an interactive viewer, similar to `top`, `htop` and other *top utilities,
but about batteries installed in your notebook.

## Example

![Screenshot](https://raw.githubusercontent.com/svartalf/rust-battop/master/assets/screenshot.png)

## Compatibility

`battop` supports *nix systems, MacOS and Windows.

## Installation

### Arch linux

Install package from [AUR](https://aur.archlinux.org/packages/battop/) with your favorite AUR helper:

```
$ yay -S battop
```

### From sources

Clone the repo and run

```
$ cargo build --release
```

### Other

Prebuilt binaries for Linux, FreeBSD, MacOS and Windows can be downloaded from the [GitHub releases page](https://github.com/svartalf/rust-battop/releases).

## License

`battop` is double-released under the Apache License, Version 2.0 or the MIT License.

## Donations

If you appreciate my work and want to support me, you can do it [here](https://svartalf.info/donate/)
