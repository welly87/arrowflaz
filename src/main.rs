use datafusion::prelude::*;
use arrow::util::pretty::print_batches;
use arrow::record_batch::RecordBatch;

#[tokio::main]
async fn main() -> datafusion::error::Result<()> {
    // create the dataframe
    let mut ctx = ExecutionContext::new();
    let df = ctx.read_csv("tests/example.csv", CsvReadOptions::new())?;

    let df = df.filter(col("a").lt_eq(col("b")))?
        .aggregate(vec![col("a")], vec![min(col("b"))])?
        .limit(100)?;

    // execute and print results
    let results: Vec<RecordBatch> = df.collect().await?;
    print_batches(&results)?;
    Ok(())
}