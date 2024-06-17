# Getting Started

For scm I use [jj](https://github.com/martinvonz/jj), feel free to not.

## Dev Setup

- Install [rustup](https://rustup.rs/)

## Faster Compiles

### Use mold (mac)

#### Note the latest mac linker is fast enough so you can skip this if on xcode 15 linker ld-prime

[Acquire a license to sold](https://bluewhale.systems/), this is only requried on mac.
See [install instructions](https://github.com/bluewhalesystems/sold/issues/26)

```bash
git clone git@github.com:bluewhalesystems/sold.git
mkdir sold/build
cd ./sold/build
# if you don't have cmake, please run:
# brew install cmake
cmake -DCMAKE_BUILD_TYPE=Release -DCMAKE_CXX_COMPILER=c++ ..
cmake --build . -j $(sysctl -n hw.ncpu)
sudo cmake --build . --target install
```

Add this to your **`~/.cargo/config.toml`**

```toml
[target.aarch64-apple-darwin]
linker = "clang"
rustflags = ["-C", "link-arg=-fuse-ld=/usr/local/bin/ld64.mold"]
```

Verify you are using mold

```bash
otool -l target/debug/<YOUR BIN> | grep 'tool '
```

### Install Cranelift

```bash
rustup update nightly #install nightly if you haven't already
rustup component add rustc-codegen-cranelift-preview --toolchain nightly
```

Add this to your **`~/.cargo/config.toml`** if its not already, or just dont delete it from this repo's `.cargo/config.toml`.

```toml
[unstable]
codegen-backend = true
[profile.server-dev]
codegen-backend = "cranelift"
```
