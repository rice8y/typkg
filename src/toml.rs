use std::fs;
use toml::Value;
use anyhow::{Context, Result};

pub fn read_name_version(path: &str) -> Result<(String, String)> {
    let content = fs::read_to_string(path)
        .with_context(|| format!("Failed to read {}", path))?;
    let value: Value = toml::from_str(&content)?;
    let name = value["package"]["name"]
        .as_str()
        .context("Missing [package] name")?
        .to_string();
    let version = value["package"]["version"]
        .as_str()
        .context("Missing [package] version")?
        .to_string();
    Ok((name, version))
}
