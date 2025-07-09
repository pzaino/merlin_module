# Merlin Module

[![Security Audit](https://github.com/pzaino/merlin_module/actions/workflows/rust-security.yml/badge.svg)](https://github.com/pzaino/merlin_module/actions/workflows/rust-security.yml)
![CodeQL: ](https://github.com/pzaino/merlin_module/actions/workflows/github-code-scanning/codeql/badge.svg)
[![FOSSA Status](https://app.fossa.com/api/projects/git%2Bgithub.com%2Fpzaino%2Fmerlin_module.svg?type=shield&issueType=security)](https://app.fossa.com/projects/git%2Bgithub.com%2Fpzaino%2Fmerlin_module?ref=badge_shield&issueType=security)
[![Build Test](https://github.com/pzaino/merlin_module/actions/workflows/rust.yml/badge.svg)](https://github.com/pzaino/merlin_module/actions/workflows/rust.yml)
[![Rust](https://img.shields.io/badge/rust-1.70+-blue.svg)](https://www.rust-lang.org/)

[![License](https://img.shields.io/badge/license-MPL%202.0-blue.svg)](LICENSE.txt)
[![FOSSA Status](https://app.fossa.com/api/projects/git%2Bgithub.com%2Fpzaino%2Fmerlin_module.svg?type=shield&issueType=license)](https://app.fossa.com/projects/git%2Bgithub.com%2Fpzaino%2Fmerlin_module?ref=badge_shield&issueType=license)

![GitHub language count](https://img.shields.io/github/languages/count/pzaino/merlin_module)
![GitHub commit activity](https://img.shields.io/github/commit-activity/t/pzaino/merlin_module)
![GitHub contributors](https://img.shields.io/github/contributors/pzaino/merlin_module)
![GitHub code search count](https://img.shields.io/github/search?query=merlin_module)

Status: **WORK IN PROGRESS**

## Description

The Merlin module is a Rust library designed to facilitate the development of Merlin Kernel's modules. It provides a set of utilities and macros that simplify the process of creating a kernel module.

If you don't know what RISC OS Merlin is or you are looking for the Merlin Kernel itself, please refer to the [Project Presentation](https://riscoscommunity.org/projects/risc-os-merlin/).

## Features

- **Header Management**: Automatically generates and manages headers for kernel modules.
- **Constants**: Provides a set of constants that are commonly used in kernel module development.
- **Macros**: Includes macros to simplify common tasks in module development.

## Usage

To use the Merlin module in your project, add the following to your `Cargo.toml`:

```toml
[dependencies]
merlin_module = { git = "https://github.com/pzaino/merlin_module.git" }
```

Then, in your Rust code, you can import the module:

```rust
extern crate merlin_module;
use merlin_module::{consts, header, macros};
```

## Contributing

We welcome contributions to the Merlin module! If you have suggestions, improvements, or bug fixes, please open an issue or submit a pull request on the [GitHub repository](https://github.com/pzaino/merlin_module).

## License

This project is Copyright by Paolo Fabio Zaino (all rights reserved) and is licensed under the MPL 2.0 License. See the [LICENSE](LICENSE.txt) file for details.
