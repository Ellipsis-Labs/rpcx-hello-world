# RpcX Hello World Program

A simple guide to getting started with rpcX (RPC Extensions) from Ellipsis Labs.

## Setup

You will need to install the atlas-rpcx CLI tool:

Get the latest version of the rpcX CLI from the [Atlas release repo](https://github.com/Ellipsis-Labs/atlas-release/releases). 
Currently only Linux and MacOS are supported.

And setup Rust to properly develop rpcX packages:
```shell
cargo install cargo-component
rustup target add wasm32-wasip1
```

### Program

It is recommended that the sample program be built with solana-cli
version 2.0.24. To build the program, run:

```shell
cargo build-sbf
```

### Script

Run the following command to interact with the hello world program:

```shell
$ cargo run <PROGRAM_ID> <PAYER_KEYPAIR_PATH> <RPC_URL>
Transaction confirmed: 5Dq9Nc2gHpUTQ6EyP8w7HG53p5E8sG2qSiH1htY7NZvByzYNTPtd4c3bZck4kXteL8CgGwAwKb3JEy69r415r8gK
```

### rpcX Package

Run the following command to build the rpcX package:

```shell
$ cargo component build --release
Finished `release` profile [optimized] target(s) in 3.21s
Creating component target/wasm32-wasip1/release/rpcx_package.wasm
```

#### Testing a local rpcX Package
To test a local rpcX package use the `atlas-rpcx` CLI tool's `simulate` command:

```shell
$ atlas-rpcx simulate --help
```

#### Package Deployment
To deploy the rpcX package:

```shell
$ atlas-rpcx registry deploy
Finished `release` profile [optimized] target(s) in 3.21s
Creating component target/wasm32-wasip1/release/rpcx_package.wasm
```

#### Interacting with a deployed rpcX Package
To interact with a deployed rpcX package use the `atlas-rpcx` CLI tool's `query` and `pubsub` commands:

```shell
$ atlas-rpcx query --help
$ atlas-rpcx pubsub --help
```