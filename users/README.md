# Installed request

[LLVM](https://releases.llvm.org/download.html). (tthis is needed for
the [argonautica](https://crates.io/crates/argonautica) crate to work).
> Add LLVM path `C:\Program Files\LLVM\bin` into environment variable in Window

---

## Command Line

```shell
cargo run --bin users
cargo run --bin users --release
```

### Cargo uses conventions for file placement to make it easy to dive into a new Cargo package:

```
├── benches/
├── docker/
├── examples/
├── src/
│ ├── app/
│ ├── config/
│ ├── databases/
│ ├── utils/
│ ├── lib.rs
│ ├── main.rs
│ ├── app.rs
│ └── utils.rs
├── tests/
├── .env
├── .env.example
├── Cargo.toml
├── diesel.toml
└── README.md

```