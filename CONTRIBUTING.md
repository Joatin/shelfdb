# Contributing

Hi and welcome to our contribution guidelines

## Prerequisites

#### Rust
Youn need to have rust installed. We also recommend that you add Clippy and Rust-FMT to cargo

#### Node (Optional)
You need to have node installed if you wish to contribute to our documentation which is written using GatsbyJS. We also use node to set up some commit hooks, in order to automatically run some tests and formatting on each commit

#### Docker (Optional)
Shelf is deployed using docker. If you wish to test that setup you need to have docker installed

## Running
You can run Shelfdb in develop mode by simply running

```shell script
cargo run
```

This command will install all dependencies, compile the binary (might take some time), and the run it
