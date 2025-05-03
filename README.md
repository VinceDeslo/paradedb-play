# paradedb-play

Just a small repo to play around with [paradedb](https://github.com/paradedb/paradedb).

```shell
direnv allow .
just up

# Eg. connect and grab a query to run
just connect
cat sql/mock_data.sql | pbcopy

# Eg. run program that attempts full text search
just fts
```
