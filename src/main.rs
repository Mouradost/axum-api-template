use axum_api_template::{run, Result, args::Cli};
use clap::Parser;


// cargo watch -q -c -w src/ -x run
#[tokio::main]
async fn main() -> Result<()>{
    let args = Cli::parse();
    run(args.port, args.log_level, args.log_output, args.max_connections).await?;
    Ok(())
}
