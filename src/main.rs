use anyhow::Result;
use clap::Parser;

use cli::{Cli, Commands};
use database::migrate::migrate;
use http::start_server;
use stressors::sensor::sensor_stress;
use stressors::stress::application_stress;

mod cli;
mod database;
mod http;
mod log;
mod model;
mod repositories;

mod stressors;

#[actix_web::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();

    log::init();

    match &cli.command {
        Commands::Server(args) => start_server(args).await,
        Commands::Migrate {
            config,
            drop_keyspace,
        } => migrate(config, drop_keyspace.clone()).await,
        Commands::Sensor {
            config,
            measure,
            buffer_interval,
        } => sensor_stress(config, measure, buffer_interval).await,
        Commands::Stress { config, stress } => application_stress(config, stress).await,
    }
}
