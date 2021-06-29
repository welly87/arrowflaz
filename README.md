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