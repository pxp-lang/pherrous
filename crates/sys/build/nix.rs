use std::path::PathBuf;

use anyhow::Result;
use which::which;

use crate::Provider;

pub struct PlatformProvider;

impl Provider for PlatformProvider {
    fn new() -> Self {
        Self
    }
    
    fn includes() -> Result<Vec<PathBuf>> {
        Ok(vec![])
    }

    fn defines() -> Result<Vec<(String, String)>> {
        Ok(vec![])
    }

    fn php_binary() -> Result<PathBuf> {
        which("php").map_err(|err| anyhow::anyhow!(err))
    }

    fn php_config_binary() -> Result<PathBuf> {
        which("php-config").map_err(|err| anyhow::anyhow!(err))
    }
}