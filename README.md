# battop

[![Latest Version](https://img.shields.io/crates/v/battop.svg)](https://crates.io/crates/battop)
[![Build Status](https://travis-ci.org/svartalf/rust-battop.svg?branch=master)](https://travis-ci.org/svartalf/rust-battop)
[![dependency status](https://deps.rs/crate/battop/0.2.4/status.svg)](https://deps.rs/crate/battop/0.2.4)
![Apache 2.0 OR MIT licensed](https://img.shields.io/badge/license-Apache2.0%2FMIT-blue.svg)

`battop` is an interactive viewer, similar to `top`, `htop` and other *top utilities,
but about the batteries installed in your notebook.

![Screenshot](https://raw.githubusercontent.com/svartalf/rust-battop/master/assets/screenshot.png)

## Features

 * Cross-platform (Linux, MacOS, FreeBSD and DragonflyBSD are supported and Windows is [on the way](https://github.com/svartalf/rust-battop/issues/5))
 * Supports multiple batteries in case your notebook have them 
 * It is free
 * Usually it just works!

`battop` is backed by a Rust crate [battery](https://crates.io/crates/battery)
which provides unified cross-platform information about system batteries.\
[Check it out](https://github.com/svartalf/rust-battery),
if you want to gather the same information for your application!

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

Prebuilt binaries for Linux, FreeBSD and MacOS can be downloaded from the [GitHub releases page](https://github.com/svartalf/rust-battop/releases).

## Usage

Simply running the `battop` command in your terminal should do the thing.

Left and right arrows can be used to switch between different system batteries (if available).

Run the `battop -h` command to see the additional available options.

## License

`battop` is double-released under the Apache License, Version 2.0 or the MIT License.

## Donations

If you appreciate my work and want to support me, you can do it [here](https://svartalf.info/donate/)
