# versatus-rust

### Overview

This respository provides some essential tools and interfaces for developing **Programs** for the **LASR** network using Rust.
It is built using using provided types from the `lasr_types` crate, with examples and functions to aid in the building of LASR programs.
Along with some helper functions the `lasr-rust` CLI is used to initalize, test, deploy,  and call programs in the network from the terminal.

#### Learn more about LASR

- [Versatus Website](https://versatus.io)

# Getting Started
#### Dependencies
* Rust _(>= v1.75.0)
* lasr_cli (built from the [versatus/lasr](https://github.com/versatus/lasr) repository)

#### 1) Clone and build `versatus/versatus-rust` repository
```bash
git clone https://github.com/versatus/versatus-rust.git
cd versatus-rust/lasr
cargo build
```
This will give you access to the `lasr-rust` CLI you will be using.

#### 2) Create New Rust Project Directory
```bash
mkdir your-project-name
cd your-project-name
```

#### 3) Initialize Project with lasrctl
```bash
~/PATH/TO/lasr-rust init --blank
```
This will generate files necessary for building and deployment of a LASR program in Rust.
Including:
  - `src/main.rs`
  - `Cargo.toml`
  - `.lasr/keypair.json`
  - `example-program-inputs.json`

#### 4) Modify `your-project-name`
In your newly initalized rust project, add the `lasr-rust` dependency in your `Cargo.toml` file like so:
```
[dependencies]
lasr-rust = { path = "PATH/TO/CLONED/REPO" }
```
Next some optimizations need to be added to lower the binary size of your program.
Add this to your `Cargo.toml` file:
```
[profile.release]
opt-level = 'z'     # Optimize for size
lto = true          # Enable link-time optimization
codegen-units = 1   # Reduce number of codegen units to increase optimizations
panic = 'abort'     # Abort on panic
strip = true        # Strip symbols from binary*
```
The last step is to modify a single import in the generated `main.rs` file.
Located at the top of the file, just replace `crate::` with `lasr_rust::` to avoid a compilation error.

#### 5) Build `your-project-name`
In your rust project directory, build your project with the `--release` flag.
```bash
cargo build --release
```

#### 6) Test `your-project-name`
Using the `lasr-rust` CLI, run the following command:
```bash

```
