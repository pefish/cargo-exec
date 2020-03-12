# cargo-exec

`cargo-exec` brings the [`npm run`](https://docs.npmjs.com/cli/run-script) functionality to the Rust and Cargo ecosystem. 

## Install

```bash
cargo install cargo-exec
```

## Defining Scripts

Scripts are defined by adding a `[package.metadata.scripts]` section to the `Cargo.toml` file of your project, as shown below.

```toml
[package.metadata.scripts]
hello = "echo Hello"
```

## Running Scripts

```bash
cargo exec hello
```