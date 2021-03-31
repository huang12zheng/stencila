use crate::convert;
use crate::open;
use crate::plugins;
use crate::request;
use crate::serve;
use crate::upgrade;
use crate::validate;
use crate::{config, plugins::read_plugins};
use anyhow::Result;
use structopt::StructOpt;
use strum::{Display, EnumVariantNames};

#[derive(Debug, StructOpt)]
#[structopt(
    about = "Stencila command line tool",
    setting = structopt::clap::AppSettings::DeriveDisplayOrder
)]
pub struct Args {
    #[structopt(subcommand)]
    pub command: Command,
}

#[derive(Debug, Display, StructOpt, EnumVariantNames)]
#[strum(serialize_all = "lowercase")]
#[structopt(
    setting = structopt::clap::AppSettings::DeriveDisplayOrder
)]
pub enum Command {
    Open(open::cli::Args),
    Convert(convert::cli::Args),
    Validate(validate::cli::Args),
    Serve(serve::cli::Args),
    Request(request::cli::Args),
    Upgrade(upgrade::cli::Args),
    Plugins(plugins::cli::Args),
    Config(config::cli::Args),
}

pub async fn cli(args: Vec<String>) -> Result<i32> {
    // Parse args into a command
    let Args { command } = Args::from_iter(args);

    // If not explicitly upgrading then run an upgrade check in the background
    let upgrade_thread = if let Command::Upgrade(_) = command {
        None
    } else {
        Some(upgrade::upgrade_auto())
    };

    // Load plugins
    read_plugins()?;

    // Run the command
    let result = match command {
        Command::Open(command) => open::cli::open(command).await,
        Command::Convert(command) => convert::cli::convert(command),
        Command::Validate(command) => validate::cli::validate(command),
        Command::Serve(command) => serve::cli::serve(command).await,
        Command::Request(command) => request::cli::request(command).await,
        Command::Upgrade(command) => upgrade::cli::upgrade(command),
        Command::Plugins(command) => plugins::cli::plugins(command).await,
        Command::Config(command) => config::cli::config(command),
    };

    // Join the upgrade thread and log any errors
    if let Some(upgrade_thread) = upgrade_thread {
        if let Err(_error) = upgrade_thread.join() {
            // TODO: Log error
        }
    }

    match result {
        Ok(_) => Ok(exitcode::OK),
        Err(error) => {
            // Write the error to the terminal
            // TODO Send this to a logger
            eprintln!("{}", error);
            Ok(exitcode::SOFTWARE)
        }
    }
}
