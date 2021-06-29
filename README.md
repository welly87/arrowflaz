
## Arrow Fusion
https://github.com/apache/arrow-datafusion/blob/master/datafusion/docs/cli.md

```shell
git clone https://github.com/apache/arrow-datafusion
cd arrow-datafusion/datafusion-cli
cargo run --release
```

```
datafusion-cli 
```

```sql
CREATE EXTERNAL TABLE coinbase
STORED AS PARQUET
LOCATION '/tmp/coinbase-all.parquet';
```

```sql
show columns from coinbase;
```

```sql
select count(*) from coinbase;
```

```sql
select avg(price), sum(amount) from coinbase;
```


## Balista
https://ballistacompute.org/docs/