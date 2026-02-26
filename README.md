# adbc-rust-deps-test

Test repo to figure out some dependency resolution issues.

This doesn't work out of the box due to incompatible types:

```sh
cargo init
cargo add adbc_core adbc_driver_manager arrow parquet
cargo run
```

I get:

```sh
$ cargo run
   Compiling adbc-rust-deps-test v0.1.0 (/Users/bryce/src/amoeba/adbc-rust-deps-test)
error[E0308]: `?` operator has incompatible types
   --> src/main.rs:43:9
    |
 43 |         reader.collect::<std::result::Result<Vec<_>, _>>()?;
    |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected `arrow::array::RecordBatch`, found `arrow_array::record_batch::RecordBatch`
    |
    = note: `?` operator cannot convert from `Vec<arrow_array::record_batch::RecordBatch>` to `Vec<arrow::array::RecordBatch>`
note: two different versions of crate `arrow_array` are being used; two types coming from two different versions of the same crate are different types even if they look the same
   --> /Users/bryce/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/arrow-array-58.0.0/src/record_batch.rs:202:1
    |
202 | pub struct RecordBatch {
    | ^^^^^^^^^^^^^^^^^^^^^^ this is the expected type `arrow::array::RecordBatch`
    |
   ::: /Users/bryce/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/arrow-array-57.3.0/src/record_batch.rs:202:1
    |
202 | pub struct RecordBatch {
    | ^^^^^^^^^^^^^^^^^^^^^^ this is the found type `arrow_array::record_batch::RecordBatch`
    |
   ::: src/main.rs:3:5
    |
  3 | use adbc_core::options::{IngestMode, OptionDatabase, OptionStatement};
    |     --------- one version of crate `arrow_array` used here, as a dependency of crate `adbc_core`
...
  7 | use parquet::arrow::arrow_reader::ParquetRecordBatchReaderBuilder;
    |     ------- one version of crate `arrow_array` used here, as a dependency of crate `parquet`
    = help: you can use `cargo tree` to explore your dependency tree

error[E0277]: the trait bound `ParquetRecordBatchReader: arrow_array::record_batch::RecordBatchReader` is not satisfied
    --> src/main.rs:59:27
     |
  59 |     statement.bind_stream(Box::new(reader))?;
     |                           ^^^^^^^^^^^^^^^^ the trait `arrow_array::record_batch::RecordBatchReader` is not implemented for `ParquetRecordBatchReader`
     |
note: there are multiple different versions of crate `arrow_array` in the dependency graph
    --> /Users/bryce/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/arrow-array-57.3.0/src/record_batch.rs:30:1
     |
  30 | pub trait RecordBatchReader: Iterator<Item = Result<RecordBatch, ArrowError>> {
     | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ this is the required trait
     |
    ::: src/main.rs:3:5
     |
   3 | use adbc_core::options::{IngestMode, OptionDatabase, OptionStatement};
     |     --------- one version of crate `arrow_array` used here, as a dependency of crate `adbc_core`
...
   7 | use parquet::arrow::arrow_reader::ParquetRecordBatchReaderBuilder;
     |     ------- one version of crate `arrow_array` used here, as a dependency of crate `parquet`
     |
    ::: /Users/bryce/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parquet-58.0.0/src/arrow/arrow_reader/mod.rs:1333:1
     |
1333 | pub struct ParquetRecordBatchReader {
     | ----------------------------------- this type doesn't implement the required trait
     |
    ::: /Users/bryce/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/arrow-array-58.0.0/src/array/dictionary_array.rs:1005:1
     |
1005 | pub trait AnyDictionaryArray: Array {
     | ----------------------------------- this is the found trait
     = help: you can use `cargo tree` to explore your dependency tree
     = note: required for the cast from `Box<ParquetRecordBatchReader>` to `Box<(dyn arrow_array::record_batch::RecordBatchReader + Send + 'static)>`

error[E0308]: `?` operator has incompatible types
   --> src/main.rs:69:9
    |
 69 |         reader.collect::<std::result::Result<Vec<_>, _>>()?;
    |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected `arrow::array::RecordBatch`, found `arrow_array::record_batch::RecordBatch`
    |
    = note: `?` operator cannot convert from `Vec<arrow_array::record_batch::RecordBatch>` to `Vec<arrow::array::RecordBatch>`
note: two different versions of crate `arrow_array` are being used; two types coming from two different versions of the same crate are different types even if they look the same
   --> /Users/bryce/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/arrow-array-58.0.0/src/record_batch.rs:202:1
    |
202 | pub struct RecordBatch {
    | ^^^^^^^^^^^^^^^^^^^^^^ this is the expected type `arrow::array::RecordBatch`
    |
   ::: /Users/bryce/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/arrow-array-57.3.0/src/record_batch.rs:202:1
    |
202 | pub struct RecordBatch {
    | ^^^^^^^^^^^^^^^^^^^^^^ this is the found type `arrow_array::record_batch::RecordBatch`
    |
   ::: src/main.rs:3:5
    |
  3 | use adbc_core::options::{IngestMode, OptionDatabase, OptionStatement};
    |     --------- one version of crate `arrow_array` used here, as a dependency of crate `adbc_core`
...
  7 | use parquet::arrow::arrow_reader::ParquetRecordBatchReaderBuilder;
    |     ------- one version of crate `arrow_array` used here, as a dependency of crate `parquet`
    = help: you can use `cargo tree` to explore your dependency tree

Some errors have detailed explanations: E0277, E0308.
For more information about an error, try `rustc --explain E0277`.
error: could not compile `adbc-rust-deps-test` (bin "adbc-rust-deps-test") due to 3 previous errors
```
