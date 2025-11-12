# commi

Monorepo for the commi project. It contains a SvelteKit frontend and a Rust-based authentication service that implements Discord OAuth2, session storage in Redis, token encryption, and Postgres persistence.

The Auth Service needs the DB to be setup to compile since we need compile time checks.
docker compose up -d auth_db

Get Sqlx
cargo install sqlx-cli

Setup the Database Tables etc.
sqlx migrate run --database-url postgresql://user:password@localhost:port/db_name from the env
cargo sqlx prepare --database-url postgresql://user:password@localhost:port/db_name from the env

Now we can start we rest of the containers.
docker compose up -d
