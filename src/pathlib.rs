use std::{fs, path::Path};

use anyhow::{bail, Result};

pub trait PyPath {
    fn mkdir(&self, parents: bool, exist_ok: bool) -> Result<()>;
}

impl PyPath for Path {
    fn mkdir(&self, parents: bool, exist_ok: bool) -> Result<()> {
        if !exist_ok && self.exists() {
            bail!("File exists: '{}'", self.display());
        }

        if exist_ok && self.is_dir() {
            return Ok(());
        }

        if parents {
            fs::create_dir_all(self)?;
        } else {
            fs::create_dir(self)?;
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use tempfile::tempdir;

    #[test]
    fn both_false_ok() -> Result<()> {
        let temp_dir = tempdir()?;

        let mock_dir_path = temp_dir.path().join("mock_dir");
        assert!(!mock_dir_path.exists());

        mock_dir_path.mkdir(false, false)?;
        assert!(mock_dir_path.is_dir());

        drop(mock_dir_path);
        temp_dir.close()?;

        Ok(())
    }
}
