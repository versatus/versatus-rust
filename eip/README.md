# versatus-rust

## Overview

This crate provides some helper code and interfaces for developing Versatus smart contracts in the Rust programming language. We maintain a [high-level and language-agnostic overview of smart contract development on Versatus](https://github.com/versatus/versatus/blob/main/docs/DevelopingSmartContracts.md) that is worth reading in conjunction with the documentation in this repository that covers the Rust-specifics.

The smart contract input and output data is serialised/deserialised to/from Rust data structures, which are all documented inline with `rustdoc` and browseable with your favourite Rust Doc tools, including [crates.io](https://crates.io/crates/versatus-rust).

## Rust Installation

The [Official Rust Docs](https://www.rust-lang.org/) provide instructions for the installation and maintenance of Rust installations for a number of different supported use cases. Versatus has been careful to not rely on too much bleeding-edge Rust functionality and should be compatible with whichever version of Rust you're likely to be using.

## Rust Dependencies

We want to keep the dependencies required down to a minimum too. Smart contracts on Versatus are compiled not to machine instructions like a regular program, but to Web Assembly instructions to allow it to be executed across any platform on the Versatus network, including standalone on your development machine, using our smart contract runtime. The Rust community provides the necessary compiler backend to compile your Rust code to WASM that is compatible with the Versatus smart contract runtime.

To add this WASM target to rust, you can use the standard `rustup` tool to add the `wasm32-wasi` target:

```
rustup target add wasm32-wasi
```

## Developing a Smart Contract

To use this crate to develop a smart contract, simply import this crate as you would any other crate (eg `cargo add versatus-rust`), and define your contract from there.

The [ERC20](examples/erc20.rs) example shows an example of how to build your own ERC20 token on the Versatus network using the provided `Erc20` trait. All we need is a Rust `main()` function to define the new token type, and to call a helper function to process the inputs/outputs and to call the requested function:

```rust
fn main() {
    let mut token = ComputeUnitToken { inputs: None };
    process_erc20(&mut token).unwrap();
}
```

In this case, the `ComputeInitToken` implements the `SmartContract` and `Erc20` traits defined in this crate. All that's left for the developer to do is to implement the required functions/methods defined by those traits.

See the [ERC20](examples/erc20.rs) example and the [crate docs](https://crates.io/crates/versatus-rust/) for specific details.

## Building a Smart Contract

Compiling your smart contract code to a WASM smart contract for Versatus is the same as building any other Rust project using the `cargo build` command. All that's needed is to specify the `wasm32-wasi` target when building your project. This can either be done on the command line each time (don't forget!) or may be set in your [config.toml](.cargo/config.toml) file. From the command line, just include the `--target` options:

```
cargo build --target wasm32-wasi
```

To build the [ERC20](examples/erc20.rs) example contract, you can use the standard `cargo build --example` command:


```
cargo build --example erc20
```

Note that in this case, we didn't specify the target to be `wasm32-wasi` on the command line. This is because this crate's [config.toml](.cargo/config.toml) file sets the default target.

In both cases, your `target/` directory should contain the compiled WASM file for your project. In the case of the example above, this would likely be something like `target/examples/erc20.wasm`.

## Testing a Smart Contract
[The language-agnostic smart contract development guide](https://github.com/versatus/versatus/blob/main/docs/DevelopingSmartContracts.md) contains more in-depth details for testing your smart contract in isolation, but as a simple smoke test, you can download the `versatus-wasm` binary for your platform, create or use some sample JSON input, and execute your smart contract with the `versatus-wasm` runtime. To test the [ERC20](examples/erc20.rs) contract above, you could execute the following:

```
./versatus-wasm --wasm target/examples/erc20.wasm --json ./sample-input.json
```

Where the `./sample-input.json` file might look something like this:

```json
{
  "version": 1,
  "accountInfo": {
    "accountAddress": "0x0202020202020202020202020202020202020202",
    "accountBalance": "0xffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff"
  },
  "protocolInput": {
    "version": 1,
    "blockHeight": 1,
    "blockTime": 1
  },
  "contractInput": {
    "contractFn": "name",
    "functionInputs": {
      "erc20": {
        "name": []
      }
    }
  }
}
```


