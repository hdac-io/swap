# Swap contract

[![Build Status](https://travis-ci.com/hdac-io/swap.svg?branch=master)](https://travis-ci.com/hdac-io/swap)

## Build the contract

### Supported systems

Automated test runs and we guarantee to support these systems below:

Ubuntu 18.04 or later
MacOS 10.14 or later

### Prerequisites

* [Rust](https://www.rust-lang.org/tools/install)
* [protoc](http://google.github.io/proto-lens/installing-protoc.html) >= 3.6.1
* make
* cmake

### Build

```bash
make setup
make build/swap-install
```

### Install

On the running blockchain node process `nodef`

```bash
clif contract run wasm ./swap_install.wasm '' 0.1 --from elsa
```

### Test

```bash
make test
```

## Documents

* [Swap process](docs/Process.md)
* [Method lists](docs/Methods.md)
