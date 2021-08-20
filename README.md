# jjvm

jjvm is an attempt at implementing the JVM in Rust. It is primarily a project to learn the internals of the JVM.

## Crates
* jjvm - CLI using [clap](https://github.com/clap-rs/clap) for interacting with jjvm.
* jjvm-loader - Crate for loading in Class files.
* jjvm-vm - Crate for executing the loaded Class files.