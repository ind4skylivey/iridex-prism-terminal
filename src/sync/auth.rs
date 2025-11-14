use std::fs;
use std::path::PathBuf;

use crate::ensure_config_dir;
use crate::error::PrismResult;

const AUTH_FILE: &str = "auth.token";
const JWT_SECRET_FILE: &str = "auth.jwt";

pub fn auth_path() -> PrismResult<PathBuf> {
    Ok(ensure_config_dir()?.join(AUTH_FILE))
}

pub fn read_token() -> PrismResult<Option<String>> {
    let path = auth_path()?;
    if path.exists() {
        Ok(Some(fs::read_to_string(path)?.trim().to_string()))
    } else {
        Ok(None)
    }
}

pub fn write_token(token: &str) -> PrismResult<()> {
    let path = auth_path()?;
    fs::write(path, token)?;
    Ok(())
}

pub fn delete_token() -> PrismResult<()> {
    let path = auth_path()?;
    if path.exists() {
        fs::remove_file(path)?;
    }
    Ok(())
}

pub fn resolve_token() -> PrismResult<Option<String>> {
    if let Ok(token) = std::env::var("PRISM_SYNC_TOKEN") {
        return Ok(Some(token));
    }
    read_token()
}

pub fn write_jwt_secret(secret: &str) -> PrismResult<()> {
    let path = ensure_config_dir()?.join(JWT_SECRET_FILE);
    fs::write(path, secret)?;
    Ok(())
}

pub fn resolve_jwt_secret() -> PrismResult<Option<String>> {
    if let Ok(secret) = std::env::var("PRISM_SYNC_JWT_SECRET") {
        return Ok(Some(secret));
    }
    let path = ensure_config_dir()?.join(JWT_SECRET_FILE);
    if path.exists() {
        Ok(Some(fs::read_to_string(path)?.trim().to_string()))
    } else {
        Ok(None)
    }
}
