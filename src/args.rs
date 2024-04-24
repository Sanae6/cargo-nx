use clap::{builder::PossibleValuesParser, Args, Parser, Subcommand};
use std::{fmt, path::PathBuf};

/// The supported Rust editions
pub static SUPPORTED_EDITIONS: &[&str] = &["2015", "2018", "2021"];
/// Which Rust edition to use by default
pub static DEFAULT_EDITION: &str = "2021";
/// Which package type to use by default
pub static DEFAULT_PACKAGE_TYPE: &str = "nro";

#[derive(Parser)]
#[clap(name = "cargo", bin_name = "cargo")]
pub enum Cargo {
    Nx(CargoNxArgs),
}

#[derive(Args)]
#[clap(author, version, about)]
pub struct CargoNxArgs {
    #[clap(subcommand)]
    pub subcommand: CargoNxSubcommand,
}

#[derive(Subcommand)]
pub enum CargoNxSubcommand {
    New(CargoNxNew),
    Build(CargoNxBuild),
}

#[derive(Args)]
#[clap(about = "Create a new Rust project for the Nintendo Switch")]
pub struct CargoNxNew {
    /// Select the package type that will be built by this project.
    #[clap(short = 't', long = "type", value_enum, default_value = DEFAULT_PACKAGE_TYPE)]
    pub kind: PackageKind,
    /// Set the Rust edition to use.
    #[clap(short, long, value_parser = PossibleValuesParser::new(SUPPORTED_EDITIONS), default_value = DEFAULT_EDITION)]
    pub edition: String,
    /// Set the name of the newly created package.
    /// The path directory name is used by default.
    #[clap(short, long)]
    pub name: Option<String>,
    /// The path where the new package will be created
    #[clap(value_parser, value_name = "DIR")]
    pub path: PathBuf,
}

#[derive(Args)]
#[clap(about = "Build a Rust project for the Nintendo Switch")]
pub struct CargoNxBuild {
    /// Builds using the release profile.
    #[clap(short, long)]
    pub release: bool,
    /// The path to the project to build.
    #[clap(short, long, default_value = ".", value_name = "DIR", value_parser)]
    pub path: PathBuf,
    /// The custom target triple to use, if any.
    #[clap(short, long)]
    pub target: Option<String>,
    /// The package to build (if multiple are available)
    #[clap(short='k', long)]
    pub package: Option<String>,
    /// Displays extra information during the build process.
    #[clap(short, long)]
    pub verbose: bool,
}

#[derive(Debug, Copy, Clone, ValueEnum)]
#[clap(rename_all = "lower")]
pub enum PackageKind {
    Lib,
    Nro,
    Nsp,
}

impl fmt::Display for PackageKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let fmt_str = match self {
            PackageKind::Lib => "lib",
            PackageKind::Nro => "nro",
            PackageKind::Nsp => "nsp",
        };

        write!(f, "{}", fmt_str)
    }
}
