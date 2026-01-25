<!-- Repository-specific Copilot instructions for AI coding agents -->
# Copilot instructions — auth monorepo

This file gives focused, actionable guidance for AI coding agents working in this repository.

- Big picture: This repo is a small monorepo with two primary components:
  - `services/auth-service` — Rust (Axum) HTTP service implementing Discord OAuth2, session storage in Redis, token encryption, and Postgres persistence.
  - `services/frontend` — SvelteKit app (Vite) providing the UI.

- Key integration points:
  - Postgres migrations live at `services/auth-service/migrations` and are applied with `sqlx::migrate!()` at runtime (`src/db.rs`).
  - Environment-driven configuration: `services/auth-service/src/config.rs` reads many required env vars (AUTH_SERVICE_PORT, AUTH_DB_URL, REDIS_URL, DISCORD_* vars, TOKEN_ENCRYPTION_KEY, FRONTEND_URL, ENVIRONMENT).
  - Encryption key: `TOKEN_ENCRYPTION_KEY` is expected as HEX and decoded to a 32-byte array (see `Config::from_env`).
  - Redis is used for session storage; the service constructs a `redis::Client` from `REDIS_URL`.
  - OAuth2 flow routes: `/discord/login`, `/discord/callback`, plus `/me`, `/refresh`, `/logout` (see `services/auth-service/src/main.rs`).

- Developer workflows (exact commands):
  - Start DB for compile-time SQLx checks: `docker compose up -d auth_db` (required before `cargo build` / `cargo sqlx prepare`).
  - Install SQLx CLI if needed: `cargo install sqlx-cli`.
  - Run migrations locally: `sqlx migrate run --database-url <DATABASE_URL>` (or let the service call `run_migrations` at startup).
  - Prepare SQLx (compile-time checks): `cargo sqlx prepare --database-url <DATABASE_URL>` — requires DB available and correct env.
  - Build/run auth service: `cargo build` / `cargo run` (ensure env vars and DB/Redis are available).
  - Frontend dev: from `services/frontend`: use `pnpm install` then `pnpm run dev` (package.json scripts: `dev`, `build`, `preview`, `check`).

- Project-specific conventions & patterns to follow when editing code:
  - Keep env-driven configuration centralized in `services/auth-service/src/config.rs` and prefer adding new settings there.
  - SQL migrations are the single source of truth; add SQL files under `services/auth-service/migrations` and avoid ad-hoc schema changes.
  - The service uses `Arc`-wrapped `AppState` (config, `PgPool`, Redis client, oauth2 client) — maintain thread-safe sharing when adding state.
  - CORS is strict: origin is constructed from `FRONTEND_URL` in `main.rs`; update CORS rules there if changing frontends.
  - Token encryption expects a 32-byte key passed as HEX — do not change encoding without updating `Config::from_env`.

- Important files to reference when making changes:
  - Service entry and routes: `services/auth-service/src/main.rs`
  - Configuration: `services/auth-service/src/config.rs`
  - DB helpers & migrations: `services/auth-service/src/db.rs` and `services/auth-service/migrations`
  - Frontend entry: `services/frontend/package.json` and `services/frontend/src` (SvelteKit routes)
  - Service docs: `services/auth-service/docs` (architecture and flows diagrams)

- When proposing code changes that touch infra or runtime behavior:
  - Ensure migrations are provided and `sqlx prepare` passes locally (DB up).
  - Update README notes in `README.md` to reflect changed run/build steps if they differ.
  - If adding new env vars, update `Config::from_env` and fail fast with clear error messages as current code does.

- If any details are missing or you want an expanded example (sample `.env`, Docker compose snippet, or example migration), tell me which area to expand.
