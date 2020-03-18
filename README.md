### Rust Examples

More to come!

#### Postgres

 - https://github.com/sfackler/rust-postgres
 - https://docs.rs/postgres/0.17.2/postgres/
 - https://hub.docker.com/_/postgres


```
docker run -d --rm -p 5432:5432 -e POSTGRES_PASSWORD=password postgres:9.6.17-alpine
```

#### Cassandra

 - https://github.com/AlexPikalov/cdrs
 - https://github.com/AlexPikalov/cdrs-helpers-derive
 - https://hub.docker.com/_/cassandra

 ```
docker run -d --rm -p 9042:9042 cassandra:3.11.6
 ```
