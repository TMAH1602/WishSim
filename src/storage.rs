use std::{fs, path::PathBuf};

use color_eyre::eyre::{Context, Result};
use directories::ProjectDirs;

use crate::model::SaveData;

fn path() -> Result<PathBuf> {
    let dirs = ProjectDirs::from("dev", "wishsim", "WishSim")
        .ok_or_else(|| color_eyre::eyre::eyre!("could not determine a data directory"))?;
    Ok(dirs.data_local_dir().join("save.json"))
}

pub fn load() -> Result<SaveData> {
    let path = path()?;
    if !path.exists() {
        return Ok(SaveData::default());
    }
    let text =
        fs::read_to_string(&path).wrap_err_with(|| format!("could not read {}", path.display()))?;
    serde_json::from_str(&text).wrap_err("save data is invalid")
}

pub fn save(data: &SaveData) -> Result<()> {
    let path = path()?;
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)?;
    }
    let temporary = path.with_extension("json.tmp");
    fs::write(&temporary, serde_json::to_vec_pretty(data)?)?;
    fs::rename(temporary, path)?;
    Ok(())
}

pub fn reset() -> Result<()> {
    let path = path()?;
    if path.exists() {
        fs::remove_file(path)?;
    }
    Ok(())
}
