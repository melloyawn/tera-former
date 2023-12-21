//! tera-former: a stupid simple static site generator

#![forbid(unsafe_code)]
#![warn(missing_docs, unreachable_pub, unused_crate_dependencies)]
#![warn(clippy::all, clippy::cargo, clippy::nursery, clippy::pedantic)]
#![warn(clippy::unwrap_used)]

use entrypoint::prelude::*;

use std::path::PathBuf;

/// CLI options
#[derive(clap::Parser, DotEnvDefault, LoggerDefault, Clone, Debug)]
#[command(version, about, long_about = None)]
pub struct CLIArgs {
    /// directory to save generated output
    #[clap(long, env)]
    output_directory: Option<PathBuf>,
    /// template source directory
    #[clap(long, env)]
    source_directory: Option<PathBuf>,
}

/// required config
#[derive(Clone, Debug)]
struct Config {
    /// directory to save generated output
    output_directory: PathBuf,
    /// template source directory
    source_directory: PathBuf,
}

impl TryFrom<CLIArgs> for Config {
    type Error = entrypoint::anyhow::Error;

    fn try_from(item: CLIArgs) -> entrypoint::anyhow::Result<Self> {
        let output_directory: PathBuf = if let Ok(dir) = std::env::var("OUTPUT_DIRECTORY") {
            dir.into()
        } else {
            item.output_directory
                .context("no output_directory supplied")?
        };

        let source_directory: PathBuf = if let Ok(dir) = std::env::var("SOURCE_DIRECTORY") {
            dir.into()
        } else {
            item.source_directory
                .context("no source_directory supplied")?
        };

        Ok(Self {
            output_directory,
            source_directory,
        })
    }
}

/// generate the static site
pub fn generate(args: CLIArgs) -> entrypoint::anyhow::Result<()> {
    let config: Config = args.try_into()?;

    info!("{:#?}", config);

    Ok(())
}
