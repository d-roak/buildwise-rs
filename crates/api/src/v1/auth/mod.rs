use crate::error::Result;

use anyhow::{anyhow, Context};
use axum::{
    extract::Request,
    routing::{delete, post},
    Router,
};

use crate::auth::{self, APIKey};

pub fn routes() -> Router {
    Router::new()
        .route("/api_key", delete(delete_api_key))
        .route("/api_key", post(create_api_key))
}

async fn create_api_key(req: Request) -> Result<String> {
    let auth_api_key = req.headers()
        .get("Authorization")
        .context("No authorization header")?
        .to_str()?;
    let api_key: APIKey = auth::get_api_key(auth_api_key.into());

    if !api_key.admin {
        return Err(anyhow!("Not authorized to create keys").into());
    }

    Ok(APIKey::new().key)
}

async fn delete_api_key(req: Request) -> Result<()> {
    let auth_api_key = req.headers()
        .get("Authorization")
        .context("No authorization header")?
        .to_str()?;
    let api_key: APIKey = auth::get_api_key(auth_api_key.into());

    // Add verification for self delete instead admin only
    if !api_key.admin {
        return Err(anyhow!("Not authorized to delete keys").into());
    }

    auth::delete_api_key();
    Ok(())
}
