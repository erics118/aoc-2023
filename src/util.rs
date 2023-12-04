use anyhow::{Context, Result};
use std::env;
use std::fs;
use std::path::PathBuf;

pub enum File {
    Problem,
    Test,
}

pub fn read_file(day: u8, file: File) -> Result<String> {
    let manifest_dir = env::var("CARGO_MANIFEST_DIR").context("CARGO_MANIFEST_DIR not set")?;

    let extension = match file {
        File::Problem => "in.txt",
        File::Test => "test.txt",
    };

    let mut path = PathBuf::from(manifest_dir)
        .join("src")
        .join("day".to_owned() + &day.to_string())
        .join("day".to_owned() + &day.to_string());

    path.set_extension(extension);

    if !path.exists() {
        anyhow::bail!("File {} does not exist", path.display());
    }

    Ok(fs::read_to_string(path)?)
}

pub fn read_in_file(day: u8) -> Result<String> {
    read_file(day, File::Problem)
}

pub fn read_test_file(day: u8) -> Result<String> {
    read_file(day, File::Test)
}
