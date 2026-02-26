# adbc-rust-deps-test

Test repo to figure out some dependency resolution issues.

This doesn't work out of the box due to incompatible types:

```sh
cargo init
cargo add adbc_core adbc_driver_manager arrow parquet
cargo run
```
