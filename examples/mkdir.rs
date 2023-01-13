use std::path::Path;

use anyhow::Result;
use rspy::pathlib::PyPath;

fn main() -> Result<()> {
    let p = Path::new("mock_dir/mock_dir2");
    p.mkdir(false, false)?;

    Ok(())
}
