// src/queries/redis/session.rs
use anyhow::Result;
use redis::AsyncCommands;
use redis::Client;
use serde::{Deserialize, Serialize};
use crate::logic::models::{OAuthSession, UserSession};

/// ------------------------ Config ------------------------
const USER_SESSION_TTL: usize = 30 * 24 * 3600; // 30 days
const OAUTH_SESSION_TTL: usize = 10 * 60; // 10 minutes

/// ------------------------ Key helpers ------------------------
fn oauth_key(session_id: &str) -> String {
    format!("oauth_session:{}", session_id)
}

fn user_key(session_id: &str) -> String {
    format!("user_session:{}", session_id)
}

/// ------------------------ OAuthSession CRUD ------------------------
pub async fn store_oauth_session(
    redis_client: &Client,
    session_id: &str,
    session: &OAuthSession,
    ttl_seconds: usize,
) -> Result<()> {
    let mut con = redis_client.get_multiplexed_async_connection().await?;
    let key = oauth_key(session_id);
    let value = serde_json::to_string(session)?;
    let _: () = con.set_ex(key, value, ttl_seconds as u64).await?;
    Ok(())
}

pub async fn get_oauth_session(
    redis_client: &Client,
    session_id: &str,
) -> Result<Option<OAuthSession>> {
    let mut con = redis_client.get_multiplexed_async_connection().await?;
    let key = oauth_key(session_id);

    if let Some(json) = con.get::<_, Option<String>>(&key).await? {
        let session: OAuthSession = serde_json::from_str(&json)?;
        Ok(Some(session))
    } else {
        Ok(None)
    }
}

pub async fn delete_oauth_session(redis_client: &Client, session_id: &str) -> Result<()> {
    let mut con = redis_client.get_multiplexed_async_connection().await?;
    let key = oauth_key(session_id);
    let _: () = con.del(key).await?;
    Ok(())
}

/// ------------------------ UserSession CRUD ------------------------
pub async fn store_user_session(
    redis_client: &Client,
    session_id: &str,
    session: &UserSession,
    ttl_seconds: usize,
) -> Result<()> {
    let mut con = redis_client.get_multiplexed_async_connection().await?;
    let key = user_key(session_id);

    // Store as JSON
    let value = serde_json::to_string(session)?;
    let _: () = con.set_ex(key, value, ttl_seconds as u64).await?;
    Ok(())
}

pub async fn get_user_session(
    redis_client: &Client,
    session_id: &str,
) -> Result<Option<UserSession>> {
    let mut con = redis_client.get_multiplexed_async_connection().await?;
    let key = user_key(session_id);

    if let Some(json) = con.get::<_, Option<String>>(&key).await? {
        let session: UserSession = serde_json::from_str(&json)?;
        Ok(Some(session))
    } else {
        Ok(None)
    }
}

pub async fn delete_user_session(redis_client: &Client, session_id: &str) -> Result<()> {
    let mut con = redis_client.get_multiplexed_async_connection().await?;
    let key = user_key(session_id);
    let _: () = con.del(key).await?;
    Ok(())
}

/// ------------------------ Optional helpers ------------------------

/// Check if a user session exists
pub async fn is_user_session_valid(redis_client: &Client, session_id: &str) -> Result<bool> {
    Ok(get_user_session(redis_client, session_id).await?.is_some())
}

/// Extend TTL for a user session
pub async fn extend_user_session_ttl(
    redis_client: &Client,
    session_id: &str,
    ttl_seconds: usize,
) -> Result<()> {
    let mut con = redis_client.get_multiplexed_async_connection().await?;
    let key = user_key(session_id);
    let _: () = con.expire(key, ttl_seconds as i64).await?;
    Ok(())
}

/// Atomically update refresh token + nonce + TTL
pub async fn update_user_refresh_token(
    redis_client: &Client,
    session_id: &str,
    refresh_token: Vec<u8>,
    nonce: [u8; 12],
    ttl_seconds: usize,
) -> Result<()> {
    let mut con = redis_client.get_multiplexed_async_connection().await?;
    let key = user_key(session_id);

    // Store refresh_token and nonce as hash fields
    let _: () = con
      .hset_multiple(
          &key,
          &[("refresh_token", refresh_token), ("nonce", Vec::from(nonce))],
      )
      .await?;

    let _: () = con.expire(&key, ttl_seconds as i64).await?;
    Ok(())
}
