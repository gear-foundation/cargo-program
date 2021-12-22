# cargo-program

[![CI Status](https://github.com/gear-tech/cargo-program/workflows/CI/badge.svg)](https://github.com/gear-tech/cargo-program/actions/workflows/ci.yml?query=branch%3Amaster)
[![License](https://img.shields.io/badge/license-GPL%203.0-success)](https://github.com/gear-tech/cargo-program/blob/master/LICENSE)
[![crates.io](https://img.shields.io/crates/v/cargo-program)](https://crates.io/crates/cargo-program)

Utility to simplify Gear programs development.

⚠️ **The project is currently under active development. Some features may be unstable.**

## Install

- *(recommended)* Latest version from the repo:

```
cargo install --git https://github.com/gear-tech/cargo-program.git
```

- Stable version from [crates.io](https://crates.io/crates/cargo-program):

```
cargo install cargo-program
```
## Usage

###  Create a new Gear program

```
cargo program new my-program
```

###  Create a new async Gear program

```
cargo program new my-async-program --async
```

### Build the Gear program

```
cargo program build
```

### 🚧 *(in progress)* Run the Gear program on-chain using the local node

```
cargo program run
# or specifying the running script
cargo program run --script run.rhai
# or specifying the node's URL
cargo program run --node "ws://127.0.0.1:9944"
```

### 🚧 *(in progress)* Run tests

```
cargo program test
```

### 🚧 *(in progress)* Login to the Gear backend

```
cargo program login
# or
cargo program login --host https://my-gear-backend.tld:8123
```

### 🚧 *(in progress)* Publish the Gear program to the chain

```
cargo program publish
```

## License

**cargo-program** is licensed under [GPL v3.0 with a classpath linking exception](LICENSE).
