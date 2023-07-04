use clap::Parser;

use crate::log;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    /// Server listening port.
    #[arg(short, long, default_value_t = 8080)]
    pub port: u16,
    /// Tracing log level.
    #[arg(short, long, default_value_t = tracing::Level::INFO)]
    pub log_level: tracing::Level,
    /// Tracing log level.
    #[clap(value_enum)]
    #[arg(short = 'o', long, default_value_t = log::LoggerOutput::Stdout)]
    pub log_output: log::LoggerOutput,
    /// Database maximun number of simultanious connections.
    #[arg(short, long, default_value_t = 5)]
    pub max_connections: u32,
}
