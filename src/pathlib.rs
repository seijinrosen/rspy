//! [Python の pathlib モジュール](https://docs.python.org/ja/3/library/pathlib.html) に対応するモジュール。

use std::{fs, path::Path};

use anyhow::{bail, Result};

/// `mkdir` を実装するトレイト。
pub trait PyPath: AsRef<Path> {
    /// ディレクトリを作成します。
    ///
    /// [Python の `Path.mkdir`](https://docs.python.org/ja/3/library/pathlib.html#pathlib.Path.mkdir) に相当しますが、以下の点が異なります。
    ///
    /// - `mode` 引数は実装されていません。
    /// - `parents`, `exist_ok` は、ともに必須です。
    fn mkdir(&self, parents: bool, exist_ok: bool) -> Result<()> {
        let path = self.as_ref();

        if !exist_ok && path.exists() {
            bail!("File exists: '{}'", path.display());
        }

        if exist_ok && path.is_dir() {
            return Ok(());
        }

        if parents {
            fs::create_dir_all(path)?;
        } else {
            fs::create_dir(path)?;
        }

        Ok(())
    }
}

impl PyPath for Path {}

#[cfg(test)]
mod tests {
    use super::*;

    use std::fs::File;
    use tempfile::tempdir;

    #[test]
    fn mkdir_flat() -> Result<()> {
        let temp_dir = tempdir()?;

        let mock_dir_path = temp_dir.path().join("mock_dir");
        assert!(!mock_dir_path.exists());

        mock_dir_path.mkdir(false, false)?;
        assert!(mock_dir_path.is_dir());

        let result = mock_dir_path.mkdir(false, false);
        assert!(result.is_err());

        mock_dir_path.mkdir(false, true)?;

        drop(mock_dir_path);
        temp_dir.close()?;

        Ok(())
    }

    #[test]
    fn mkdir_nested() -> Result<()> {
        let temp_dir = tempdir()?;

        let nested_mock_dir_path = temp_dir.path().join("mock_dir").join("mock_dir2");
        assert!(!nested_mock_dir_path.exists());

        let result = nested_mock_dir_path.mkdir(false, false);
        assert!(result.is_err());

        let result = nested_mock_dir_path.mkdir(false, true);
        assert!(result.is_err());

        assert!(!nested_mock_dir_path.exists());

        nested_mock_dir_path.mkdir(true, false)?;
        assert!(nested_mock_dir_path.is_dir());

        let result = nested_mock_dir_path.mkdir(false, false);
        assert!(result.is_err());

        let result = nested_mock_dir_path.mkdir(true, false);
        assert!(result.is_err());

        nested_mock_dir_path.mkdir(false, true)?;
        nested_mock_dir_path.mkdir(true, true)?;

        drop(nested_mock_dir_path);
        temp_dir.close()?;

        Ok(())
    }

    #[test]
    fn mkdir_file_exists() -> Result<()> {
        let temp_dir = tempdir()?;

        let file_path = temp_dir.path().join("mock_file");
        let file = File::create(&file_path)?;
        assert!(file_path.is_file());

        let result = file_path.mkdir(false, false);
        assert!(result.is_err());

        let result = file_path.mkdir(false, true);
        assert!(result.is_err());

        let result = file_path.mkdir(true, false);
        assert!(result.is_err());

        let result = file_path.mkdir(true, true);
        assert!(result.is_err());

        drop(file);
        temp_dir.close()?;

        Ok(())
    }
}
