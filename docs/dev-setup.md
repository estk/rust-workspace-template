# Getting Started

For scm I use [jj](https://github.com/martinvonz/jj), feel free to not.

## Dev Setup

- Install [rustup](https://rustup.rs/)

## Faster Compiles

### Install Cranelift

```bash
rustup update nightly #install nightly if you haven't already
rustup component add rustc-codegen-cranelift-preview --toolchain nightly
```
