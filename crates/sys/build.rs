use std::path::PathBuf;

use anyhow::Result;
use provider::PlatformProvider;

#[cfg_attr(not(windows), path = "build/nix.rs")]
mod provider;

pub trait Provider {
    fn new() -> Self;
    fn includes() -> Result<Vec<PathBuf>>;
    fn defines() -> Result<Vec<(String, String)>>;
    fn php_binary() -> Result<PathBuf>;
    fn php_config_binary() -> Result<PathBuf>;
    fn php_version() -> String {
        
    }
}

fn main() {
    let provider = PlatformProvider::new();
}

fn exec(cmd: &str, args: &[&str]) -> Result<String> {
    let output = std::process::Command::new(cmd)
        .args(args)
        .output()?;

    if output.status.success() {
        Ok(String::from_utf8(output.stdout)?)
    } else {
        Err(anyhow::anyhow!(String::from_utf8(output.stderr)?))
    }
}