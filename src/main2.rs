mod process;

use datafusion::prelude::*;
use arrow::util::pretty::print_batches;
use arrow::record_batch::RecordBatch;

#[tokio::main]
async fn main() -> datafusion::error::Result<()> {
    // register the table
    let mut ctx = ExecutionContext::new();
    ctx.register_csv("example", "tests/TSLA.csv", CsvReadOptions::new())?;

    // create a plan to run a SQL query
    let df = ctx.sql("SELECT AVG(Open), AVG(Close) FROM example LIMIT 100")?;

    // execute and print results
    let results: Vec<RecordBatch> = df.collect().await?;
    print_batches(&results)?;
    Ok(())
}
