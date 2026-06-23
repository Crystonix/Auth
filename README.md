# auth

Monorepo for a auth project during my study Programm. 
It contains a SvelteKit frontend and a Session based authentication service written in rust.
The infrastructure is deployed in containers by docker.

The goal was understanding the Identity Provider implementation that is normally covered by IdP/IAM Platforms such as Keycloak or Authentik.

It implements Discord OAuth2, session storage in Redis, token encryption, and Postgres persistence.

You can find a more detailed report in german here: [DiscordAuthenticator Bericht](./DiscordAuthenticator_Bericht.pdf)

## Setup

First, apply all .env variables correctly.

The Auth Service needs the DB to be setup to compile since we want compile time checks with SQLX.
Do this with:
`docker compose up -d auth_db`

Install SQLX:
`cargo install sqlx-cli`

Setup and migrate the Database Tables.
in `/services/auth-service/` run
`sqlx migrate run` , then
`cargo sqlx prepare`.

Start the rest of the containers:
`docker compose up -d`

Frontend should now be reachable via `localhost`.
