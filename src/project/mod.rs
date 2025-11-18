/// Project detection and framework-specific features
/// Detects PrestaShop, Laravel, and Symfony projects

use std::path::{Path, PathBuf};
use anyhow::Result;

#[derive(Debug, Clone, PartialEq)]
pub enum FrameworkType {
    PrestaShop,
    Laravel,
    Symfony,
    Generic,
}

pub struct Project {
    root: PathBuf,
    framework: FrameworkType,
}

impl Project {
    pub fn detect(path: &Path) -> Result<Self> {
        let framework = Self::detect_framework(path)?;
        Ok(Self {
            root: path.to_path_buf(),
            framework,
        })
    }

    fn detect_framework(path: &Path) -> Result<FrameworkType> {
        // Check for PrestaShop
        if path.join("config/defines.inc.php").exists() 
            || path.join("install/").exists() && path.join("classes/").exists() {
            return Ok(FrameworkType::PrestaShop);
        }

        // Check for Laravel
        if path.join("artisan").exists() && path.join("bootstrap/app.php").exists() {
            return Ok(FrameworkType::Laravel);
        }

        // Check for Symfony
        if path.join("bin/console").exists() && path.join("symfony.lock").exists() {
            return Ok(FrameworkType::Symfony);
        }

        Ok(FrameworkType::Generic)
    }

    pub fn framework(&self) -> &FrameworkType {
        &self.framework
    }

    pub fn root(&self) -> &Path {
        &self.root
    }
}
