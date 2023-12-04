- To start the project, pick a branch and run:

```console
cargo run
```

- The tutorial is from [CLI in Rust](https://rust-cli.github.io/book/tutorial/index.html) docs. To run it, run:

```console
cargo run -- main src/advent-of-code15/[file-name].rs
```

- To run different files, change the `[file-name]` value in `Cargo.toml`. 

---

- To log in the file that has `use log::{info, warn};` in it, run:

```
<!-- windows -->
set RUST_LOG=info

<!-- Linux and MacOs -->
env RUST_LOG=info

cargo run --bin output-log
```
