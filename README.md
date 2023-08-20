- To start the project run:

```console
cargo run
```

- The tutorial is from [CLI in Rust](https://rust-cli.github.io/book/tutorial/index.html) docs. To run it, run:

```console
cargo run -- main src/tutorial.rs
```

- To log, run:

```
<!-- windows -->
set RUST_LOG=info

<!-- Linux and MacOs -->
env RUST_LOG=info

cargo run --bin output-log
```

---

- To run different files, change the `path` value in `Cargo.toml`. 