/// Git integration for PHP Sunshine
/// Shows repository status, branch info, and file changes

use std::path::Path;
use std::process::Command;
use anyhow::Result;

#[derive(Debug, Clone, PartialEq)]
pub enum FileStatus {
    Untracked,
    Modified,
    Added,
    Deleted,
    Renamed,
    Unmodified,
}

pub struct GitRepo {
    root: String,
    current_branch: String,
}

impl GitRepo {
    pub fn open(path: &Path) -> Result<Self> {
        let root = Self::find_git_root(path)?;
        let current_branch = Self::get_current_branch(&root)?;
        
        Ok(Self {
            root,
            current_branch,
        })
    }

    fn find_git_root(path: &Path) -> Result<String> {
        let output = Command::new("git")
            .arg("-C")
            .arg(path)
            .arg("rev-parse")
            .arg("--show-toplevel")
            .output()?;
        
        if output.status.success() {
            Ok(String::from_utf8(output.stdout)?.trim().to_string())
        } else {
            anyhow::bail!("Not a git repository")
        }
    }

    fn get_current_branch(root: &str) -> Result<String> {
        let output = Command::new("git")
            .arg("-C")
            .arg(root)
            .arg("branch")
            .arg("--show-current")
            .output()?;
        
        if output.status.success() {
            Ok(String::from_utf8(output.stdout)?.trim().to_string())
        } else {
            Ok("(no branch)".to_string())
        }
    }

    pub fn branch(&self) -> &str {
        &self.current_branch
    }

    pub fn get_status(&self, file_path: &Path) -> Result<FileStatus> {
        let output = Command::new("git")
            .arg("-C")
            .arg(&self.root)
            .arg("status")
            .arg("--porcelain")
            .arg(file_path)
            .output()?;
        
        if !output.status.success() {
            return Ok(FileStatus::Unmodified);
        }

        let status_str = String::from_utf8(output.stdout)?;
        if status_str.is_empty() {
            return Ok(FileStatus::Unmodified);
        }

        let status_code = status_str.chars().take(2).collect::<String>();
        Ok(match status_code.trim() {
            "??" => FileStatus::Untracked,
            "M" | " M" | "MM" => FileStatus::Modified,
            "A" | " A" => FileStatus::Added,
            "D" | " D" => FileStatus::Deleted,
            "R" | " R" => FileStatus::Renamed,
            _ => FileStatus::Unmodified,
        })
    }

    pub fn get_diff(&self, file_path: &Path) -> Result<String> {
        let output = Command::new("git")
            .arg("-C")
            .arg(&self.root)
            .arg("diff")
            .arg(file_path)
            .output()?;
        
        Ok(String::from_utf8(output.stdout)?)
    }

    pub fn stage_file(&self, file_path: &Path) -> Result<()> {
        let status = Command::new("git")
            .arg("-C")
            .arg(&self.root)
            .arg("add")
            .arg(file_path)
            .status()?;
        
        if status.success() {
            Ok(())
        } else {
            anyhow::bail!("Failed to stage file")
        }
    }

    pub fn unstage_file(&self, file_path: &Path) -> Result<()> {
        let status = Command::new("git")
            .arg("-C")
            .arg(&self.root)
            .arg("reset")
            .arg("HEAD")
            .arg(file_path)
            .status()?;
        
        if status.success() {
            Ok(())
        } else {
            anyhow::bail!("Failed to unstage file")
        }
    }

    pub fn has_changes(&self) -> Result<bool> {
        let output = Command::new("git")
            .arg("-C")
            .arg(&self.root)
            .arg("status")
            .arg("--porcelain")
            .output()?;
        
        Ok(!output.stdout.is_empty())
    }
}
