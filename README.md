# Chess UCLV

## Setup

- Create `.env` file and write your connection url e.g.:

```txt
DATABASE_URL=postgres://user:password@127.0.0.1/chess
```

- Install `sqlx-cli` and run migrations

```sh
cargo install sqlx-cli
sqlx migrate run
```

## Run

```sh
cargo run
```

Or alternatively:

```sh
cargo run --release
```
