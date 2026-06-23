# auth

Monorepo for a auth project during my study Programm. 
It contains a SvelteKit frontend and a Session based authentication service written in rust.
The infrastructure is deployed in containers by docker. 

It implements Discord OAuth2, session storage in Redis, token encryption, and Postgres persistence.

You can find a more detailed report in german here: [DiscordAuthenticator Bericht](./DiscordAuthenticator_Bericht.pdf)

## Setup

First, apply all .env variables correctly.

The Auth Service needs the DB to be setup to compile since we need compile time checks.
Do this with:
`docker compose up -d auth_db`

Next, install SQLX:
`cargo install sqlx-cli`

Setup and migrate the Database Tables.
To do this, first go into `/services/auth-service/` and run
`sqlx migrate run` , then
`cargo sqlx prepare`.

Now we can start we rest of the containers:
`docker compose up -d`

Frontend should now be rachable vial `localhost`.
