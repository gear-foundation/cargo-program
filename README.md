# cargo-program

Utility to simplify Gear programs development.

:warning: **The project is currently under active development. Some features may be unstable.**

## Install

- [ ] *(not released yet)* Stable version from [crates.io](https://crates.io/crates/cargo-program):

```
cargo install cargo-program
```

- [x] Latest version from the repo:

```
git clone https://github.com/gear-tech/cargo-program.git
cargo install --path cargo-program
```

## Usage

###  *(in progress)* Create a new Gear program

```
cargo program new my_program
```

###  *(in progress)* Create a new async Gear program

```
cargo program new my_program --async
```

### *(in progress)* Build the Gear program

```
cargo program build
```

### *(in progress)* Run the Gear program off-chain (default)

```
cargo program run
```

### *(in progress)* Run the Gear program on-chain using local node

```
cargo program run --node
# or
cargo program run --node 127.0.0.1:9933
```

### *(in progress)* Run tests

```
cargo program test
```

### *(in progress)* Login to the Gear backend

```
cargo program login
# or
cargo program login --host https://my-gear-backend.tld:8123
```

### *(in progress)* Publish the Gear program to the chain

```
cargo program publish
```

## License

**cargo-program** is licensed under [GPL v3.0 with a classpath linking exception](LICENSE).
