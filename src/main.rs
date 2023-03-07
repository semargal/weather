//! # A CLI tool to get weather for a specific location and date

#![deny(missing_docs)]

use clap::{Args, Parser, Subcommand, ValueEnum};
use serde::{Deserialize, Serialize};
use std::process;

mod providers;

const APP_NAME: &str = "weather";

#[derive(Debug, Parser)]
#[command(name = APP_NAME)]
#[command(about = "A CLI tool to get weather for a specific location and date", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Debug, Subcommand)]
enum Commands {
    #[command(arg_required_else_help = true)]
    Configure(Provider),
    #[command(arg_required_else_help = true)]
    Get(GetArgs),
}

#[derive(Debug, Default, Clone, Serialize, Deserialize, Args)]
struct Provider {
    provider: ProviderType,
    api_key: String,
}

impl Provider {
    fn print_weather(&self, address: &str, date: &str) {
        let provider: Box<dyn providers::WeatherGetter> = match self.provider {
            ProviderType::ApiNinjas => {
                Box::new(providers::api_ninjas::ApiNinjas::new(&self.api_key))
            }
            ProviderType::WeatherApi => {
                Box::new(providers::weaterh_api::WeatherApi::new(&self.api_key))
            }
        };

        match provider.get_weather(address, date) {
            Ok(data) => println!("{data}"),
            Err(e) => {
                eprintln!("An application error occured: {e}. Please, use correct input format or try again later.");
                process::exit(1);
            }
        };
    }
}

#[derive(ValueEnum, Default, Copy, Clone, Serialize, Deserialize, Debug, PartialEq, Eq)]
enum ProviderType {
    #[default]
    WeatherApi,
    ApiNinjas,
}

#[derive(Debug, Args)]
struct GetArgs {
    address: String,
    date: Option<String>,
}

fn main() {
    let args = Cli::parse();

    match args.command {
        Commands::Configure(Provider { provider, api_key }) => {
            let config = Provider { provider, api_key };
            confy::store(APP_NAME, None, config).expect("Couldn't store configuration!");
        }
        Commands::Get(GetArgs { address, date }) => {
            let provider: Provider = confy::load(APP_NAME, None).expect(
                "Couldn't load configuration.
                Please, use `weather configure` command to configure an available provider from the list.");

            provider.print_weather(&address, date.as_deref().unwrap_or(""));
        }
    }
}
