//! This module provides the main entry point for the DuckDB CLI application.
//!
//! The application supports the following subcommands:
//!
//! - `list`: Lists the available DuckDB versions.
//! - `check`: Checks the current setup.
//! - `install`: Prompts the user to select a DuckDB version to install and installs it.
//!
//! The main function builds the command-line interface (CLI) using `build_cli` and
//! matches the subcommands provided by the user to perform the corresponding actions.

mod duckfetch;

use duckfetch::build_cli;
use duckfetch::check;
use duckfetch::duckdb_versions;
use duckfetch::install_duckdb;

use anyhow::{Context, Result};

fn main() -> Result<()> {
    let mut app = build_cli();
    let matches = app.get_matches_mut();

    match matches.subcommand() {
        Some(("list", _)) => {
            duckdb_versions()?.print_versions();
        }
        Some(("check", _)) => {
            check()?;
        }
        Some(("install", _)) => {
            let available_versions = duckdb_versions()?;
            let tag_names = available_versions.releases();
            let selected_tag = inquire::Select::new(
                "Select the DuckDB version to install (Esc to cancel): ",
                tag_names,
            )
            .prompt()
            .context("Error")?;

            let release = available_versions
                .get_release_by_tag(&selected_tag)
                .context("err")?;

            install_duckdb(release)?;
        }
        _ => {
            app.print_help()?;
            println!();
        }
    }

    Ok(())
}
