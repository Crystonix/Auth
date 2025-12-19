# Authentication & OAuth2 Architecture

## Overview

This service implements OAuth2-based authentication using Discord as the identity provider and establishes first-party, cookie-based sessions for downstream authorization.

OAuth access tokens are used only during the login flow and are not persisted beyond user identity resolution.

## Actors

* **Browser Client**: Initiates login and holds the session cookie
* **Auth Service**: Owns authentication, session creation, and authorization context
* **OAuth Provider (Discord)**: Authenticates the user and issues OAuth authorization codes
* **Redis**: Ephemeral storage for OAuth state, PKCE verifiers, and active sessions

## Trust Boundaries

* Browser ↔ Auth Service: Cookie-based authentication
* Auth Service ↔ OAuth Provider: OAuth2 Authorization Code flow with PKCE
* Auth Service ↔ Redis: Internal, trusted storage

## High-Level Authentication Flow (Mermaid)
%% include auth-flow.mmd

This diagram summarizes the full OAuth2 login and session creation lifecycle.

## Detailed Flow

### 1. Login Initiation

The client calls the login endpoint to start authentication.

The service:

* Creates an OAuth2 client configured for Discord
* Generates a CSRF state value
* Generates a PKCE verifier and challenge
* Persists state and PKCE verifier in Redis with a short TTL
* Redirects the browser to the Discord authorization endpoint

### 2. Provider Authentication

The user authenticates with Discord and grants requested scopes.

Discord redirects the browser back to the callback endpoint with:

* `code`: OAuth authorization code
* `state`: CSRF state value

### 3. OAuth Callback Validation

The callback handler:

* Extracts `code` and `state` from the request
* Validates the state against Redis
* Retrieves the PKCE verifier
* Exchanges the authorization code for an access token

### 4. Provider User Lookup

Using the access token, the service:

* Calls Discord’s user information endpoint
* Deserializes the response into the internal `DiscordUser` model

### 5. Internal User Resolution

The service maps the provider user to an internal user record:

* Existing users are resolved by provider ID
* New users are created if no match exists

A default role is assigned according to business rules.

### 6. Session Creation

The service:

* Generates a new session identifier
* Stores session data in Redis (user ID, role, expiration)
* Sets a secure, HTTP-only session cookie

OAuth tokens are discarded after this step.

### 7. Post-Login Redirect

The browser is redirected to the frontend application.

Subsequent requests are authenticated exclusively via the session cookie.

## Session Model

* Sessions are stored in Redis
* Redis is the source of truth for session validity
* Session expiration is enforced via TTL

## Security Properties

* CSRF protection via OAuth `state`
* PKCE protects against authorization code interception
* Secure, HTTP-only cookies
* Server-side session invalidation via Redis

## Error Handling

* Invalid or missing OAuth state results in authentication failure
* OAuth provider errors are handled explicitly
* Token exchange failures do not create sessions

## Invariants

* OAuth access tokens are never persisted
* A new session ID is always created after successful OAuth login
* Sessions are invalidated by deleting Redis entries
* Authentication state is never derived from client input alone

## Non-Goals

* Long-lived OAuth token storage
* Refresh-token based OAuth sessions
* Multi-provider account linking (currently unsupported)

## Open Questions / Future Work

* Explicit logout endpoint
* Session renewal or sliding expiration strategy
* CSRF protection for authenticated, non-OAuth endpoints
* Auth event auditing and observability
