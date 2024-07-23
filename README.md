# isotest-ng
Experimental reimplementation of the isotovideo module of openQA.

## What?

This repository holds three libraries: `isotomachine`, `isotoenv` (TBD) and `isototest`,
which reflect the current responsibilities of the `isotovideo` module of openQA.

This project aims to modularize and consolidate these responsibilities into easy to use
libraries.

### Design Goals

We aim to have each responsibility of this library as modularized as possible, with as small
of an interface as possible. The goal is to have the library itself handle all tasks in 
creating a test machine, setting up a test environment, and scheduling and running test with 
openQA itself only verifying the results and making sure everything is ran when it should.

In short: openQA should act as a `testmaster` or `pupeteer` which delegates the intecracies 
of each process to these libraries.

## Usage

**These libraries are currently under early development, usage - aside from development purposes - 
is heavily discouraged.**

This libraries will be published at crates.io, once they are ready.

## Contributing

If you want to contribute [please refer to our contributing guidelines](./docs/CONTRIBUTING.md).

## Security Policy

In case you have security concerns, please refer to our [security policy](./docs/SECURITY.md).
