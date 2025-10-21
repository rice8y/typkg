use std::fs;
use std::process::{Command, Stdio};
use std::time::Duration;
use tempfile::tempdir;
use anyhow::{Result, Context};
use colored::*;
use indicatif::{ProgressBar, ProgressStyle};
use fs_extra::dir::CopyOptions;
use crate::toml::read_name_version;
use crate::system::get_local_package_dir;

pub fn install(source: &str, verbose: bool) -> Result<()> {
    let tmp = tempdir()?;
    let tmp_path = tmp.path().join("repo");

    let spinner = ProgressBar::new_spinner();
    spinner.set_style(
        ProgressStyle::with_template("{spinner:.green} {msg}")
            .unwrap()
            .tick_chars("⠋⠙⠹⠸⠼⠴⠦⠧⠇⠏ ")
    );
    spinner.enable_steady_tick(Duration::from_millis(100));
    spinner.set_message("Cloning repository...");

    if source.starts_with("http") {
        let mut cmd = Command::new("git");
        cmd.args(["clone", "--depth", "1", source, tmp_path.to_str().unwrap()]);
        if !verbose {
            cmd.stdout(Stdio::null()).stderr(Stdio::null());
        }
        let status = cmd.status().context("Failed to start git clone")?;
        if !status.success() {
            spinner.finish_and_clear();
            eprintln!("{} {}", "Failed:".red().bold(), "Git clone failed");
            std::process::exit(1);
        }
    } else {
        fs::create_dir_all(&tmp_path)?;
        fs_extra::dir::copy(source, &tmp_path, &fs_extra::dir::CopyOptions::new())?;
    }

    spinner.set_message("Reading typst.toml...");
    let toml_path = tmp_path.join("typst.toml");
    let (name, version) = read_name_version(toml_path.to_str().unwrap())?;

    spinner.set_message("Copying package files...");
    let target_dir = get_local_package_dir()
        .join("local")
        .join(&name)
        .join(&version);

    fs::create_dir_all(&target_dir)?;

    let mut options = CopyOptions::new();
    options.overwrite = true;
    options.copy_inside = true;

    fs_extra::dir::copy(&tmp_path, &target_dir, &options)?;

    spinner.finish_and_clear();
    println!(
        "{} Installed {} v{} to {}",
        "Success:".green().bold(),
        name.bold(),
        version,
        target_dir.display()
    );
    Ok(())
}

pub fn clean(name: &str, version: &str) -> Result<()> {
    let dir = get_local_package_dir().join("local").join(name).join(version);
    if dir.exists() {
        fs::remove_dir_all(&dir)?;
        println!("{} Removed {}", "Success:".green().bold(), dir.display());
    } else {
        eprintln!("{} Directory not found: {}", "Failed:".red().bold(), dir.display());
    }
    Ok(())
}

pub fn list() -> Result<()> {
    let local_dir = get_local_package_dir().join("local");

    if !local_dir.exists() {
        println!("{} No packages installed", "Info:".yellow().bold());
        return Ok(());
    }

    println!("{}", "Installed Typst packages:".green().bold());
    for entry in fs::read_dir(local_dir)? {
        let entry = entry?;
        let package_name = entry.file_name().into_string().unwrap_or_default();
        let package_path = entry.path();
        if package_path.is_dir() {
            for version_entry in fs::read_dir(package_path)? {
                let version_entry = version_entry?;
                let version = version_entry.file_name().into_string().unwrap_or_default();
                println!("  {} v{}", package_name, version);
            }
        }
    }

    Ok(())
}
