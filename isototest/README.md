# IsoToTest

`isototest` is the responsible library for executing tests on `openQA` test workers.

> NOTE:
> `isototest` and its sister-libraries are highly experimental and in early stages of development. All information is subject to change.
> They are not yet ready for productive use in any capacity.

## What?

`isototest` uses the [VNC](https://wikipedia.org/wiki/Virtual_Network_Computing) protocol to communicate and interact with a given test machine. It's an asynchronous
dynamic library which will be called by the `test_master` module in openQA to run tests and evaluate their state.

## How to build

First, clone the repository. Then build the library using this command:

```
cargo build --lib --release
```

Optionally, you can enbale the default `logging` configuration by enabling the feature:

```
cargo build --lib --release --features default-logging
```

**To build the library with debug symbols, omit the `--release` flag**

Now you can use this function. To see an example, as well as the most up-to-date code documentation, let cargo build the documentation by running

```
cargo doc --lib --no-deps --document-private-items --open --features default-logging
```

This will open the freshly build documentation in you browser.

You can also use the included `Makefile` to build the library:

```
make all
```

Will build the library in release mode **without default logging enabled**. Run `make help` to get a list of other targets.

## Installation

We aim to publish all three of these libraries to `crates.io` to integrate them into the Rust crate ecosystem. Further information about
installation will follow in the future.

Currently, to install this library follow the building instructions above.

## How to contribute?

Please refer to our [contribution guide](https://github.com/os-autoinst/isotest-ng/blob/main/docs/CONTRIBUTING.md) for information on how you can
contribute to this project.
