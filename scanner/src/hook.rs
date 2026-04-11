use anyhow::{Context, Result};
use std::fs;
use std::os::unix::fs::PermissionsExt;
use std::path::PathBuf;

pub struct HookManager;

impl HookManager {
    pub fn install() -> Result<()> {
        let repo_root = Self::find_git_root()
            .context("Could not find a .git directory. Are you in a git repository?")?;

        let hooks_dir = repo_root.join(".git").join("hooks");
        if !hooks_dir.exists() {
            fs::create_dir_all(&hooks_dir)?;
        }

        let pre_commit_path = hooks_dir.join("pre-commit");
        
        let hook_content = r#"#!/bin/sh
echo "🔍 Running FSESC..."
fsesc scan .
if [ $? -ne 0 ]; then
  echo "❌ Secrets detected! Commit aborted."
  exit 1
fi
echo "✅ No secrets found."
exit 0
"#;

        fs::write(&pre_commit_path, hook_content)?;

        let mut perms = fs::metadata(&pre_commit_path)?.permissions();
        perms.set_mode(0o755);
        fs::set_permissions(&pre_commit_path, perms)?;

        Ok(())
    }

    fn find_git_root() -> Option<PathBuf> {
        let mut current = std::env::current_dir().ok()?;
        loop {
            if current.join(".git").exists() {
                return Some(current);
            }
            if !current.pop() {
                break;
            }
        }
        None
    }
}
