use crate::duckup::download_duckdb;
use crate::duckup::duckdb_versions;
use crate::duckup::extract_zip;
use anyhow::{Context, Result};
use dirs::home_dir;
use std::fs;
use std::path::Path;

/// Installs the DuckDB binary by moving it from the output directory to the install directory.
///
/// # Arguments
///
/// * `output_dir` - A `Path` representing the path to the directory containing the DuckDB binary.
/// * `install_dir` - A `Path` representing the path to the directory where the DuckDB binary should be installed.
///
/// # Returns
///
/// * `Result<()>` - An empty result if successful, or an error if the installation fails.
fn install(temp_unzip_dir: &Path, dest_path: &Path) -> Result<()> {
    let src = Path::new(temp_unzip_dir).join("duckdb");
    let dest_path = Path::new(dest_path).join("duckdb");

    fs::rename(src, dest_path).context("Failed to move DuckDB binary")?;

    Ok(())
}

/// Installs the specified version of DuckDB.
///
/// This function performs the following steps:
/// 1. Retrieves the list of available DuckDB versions.
/// 2. Checks if the requested version exists in the available versions.
/// 3. Downloads the requested version of DuckDB.
/// 4. Extracts the downloaded file.
/// 5. Installs DuckDB to the user's local bin directory.
///
/// # Arguments
///
/// * `requested_version` - A string specifying the version of DuckDB to install.
///
/// # Errors
///
/// This function will return an error if:
/// - The list of available DuckDB versions cannot be retrieved.
/// - The requested version is not found in the available versions.
/// - The DuckDB download fails.
/// - The downloaded file cannot be extracted.
/// - The home directory cannot be found.
/// - The installation process fails.
///
/// # Returns
///
/// This function returns `Ok(())` if the installation completes successfully.
pub fn install_duckdb(requested_version: String) -> Result<()> {
    let available_versions = duckdb_versions()?;

    // Check if the requested version exists in the available versions
    if !available_versions.contains_version(&requested_version) {
        eprintln!(
            "Error: Requested DuckDB version '{}' is not available. Choose one of the folowing:",
            requested_version
        );

        available_versions.print_versions();
        return Err(anyhow::anyhow!("Version not found"));
    }

    println!("Downloading DuckDB version: {} ...", requested_version);

    let (downloaded_file, temp_dir) = download_duckdb(&requested_version)?;

    println!(
        "DuckDB version {} successfully downloaded",
        requested_version
    );

    let temp_dir_str = temp_dir.path();

    extract_zip(downloaded_file, temp_dir_str)?;

    let dest_path = home_dir()
        .context("Could not find the home directory")?
        .join(".local")
        .join("bin");

    install(temp_dir_str, &dest_path)?;

    println!(
        "DuckDB {} installed successfully in {}!",
        requested_version,
        dest_path.to_str().unwrap()
    );

    Ok(())
}
