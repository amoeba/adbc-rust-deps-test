use std::fs::File;

use adbc_core::options::{IngestMode, OptionDatabase, OptionStatement};
use adbc_core::{options::AdbcVersion, Connection, Database, Driver, Statement};
use adbc_core::{Optionable, LOAD_FLAG_DEFAULT};
use adbc_driver_manager::ManagedDriver;
use parquet::arrow::arrow_reader::ParquetRecordBatchReaderBuilder;

fn main() -> std::result::Result<(), Box<dyn std::error::Error>> {
    let driver_name = "duckdb";

    // Load the driver
    let mut driver = ManagedDriver::load_from_name(
        &driver_name,
        None,
        AdbcVersion::default(),
        LOAD_FLAG_DEFAULT,
        None,
    )?;

    // Set up database options. Each database has its own set of options.
    // For databases that support URIs:
    // let opts = [(OptionDatabase::Uri, connection_uri.into())];
    // or ones that don't like
    let opts = [(
        OptionDatabase::Other("path".into()),
        "penguins.duckdb".into(),
    )];

    // Create a database
    let db = driver.new_database_with_opts(opts)?;

    // Create a connection
    let mut connection = db.new_connection()?;

    // Execute a query
    let mut statement: adbc_driver_manager::ManagedStatement = connection.new_statement()?;
    statement.set_sql_query("SELECT 1")?;
    let reader = statement.execute()?;

    // Fetch a query result as RecordBatches
    let result_batches: Vec<arrow::array::RecordBatch> =
        reader.collect::<std::result::Result<Vec<_>, _>>()?;

    // // Ingest a Parquet file
    let file = File::open("penguins.parquet")?;
    let builder = ParquetRecordBatchReaderBuilder::try_new(file)?;
    let reader = builder.build()?;

    let mut statement = connection.new_statement()?;

    // // Set the target table
    statement.set_option(OptionStatement::TargetTable, "penguins".into())?;

    // // Set the ingest mode to CreateAppend
    statement.set_option(OptionStatement::IngestMode, IngestMode::Create.into())?;

    // // Bind the data
    statement.bind_stream(Box::new(reader))?;

    // // Execute the insert
    statement.execute_update()?;

    // Debug
    let mut statement: adbc_driver_manager::ManagedStatement = connection.new_statement()?;
    statement.set_sql_query("SELECT * from penguins")?;
    let reader = statement.execute()?;
    let result_batches: Vec<arrow::array::RecordBatch> =
        reader.collect::<std::result::Result<Vec<_>, _>>()?;

    println!("result: {:?}", result_batches);
    Ok(())
}
