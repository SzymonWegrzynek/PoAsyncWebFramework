use std::error::Error;

use crate::cli::run::Cli;

mod cli;
mod models;
mod services;
mod tests;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    Cli::run().await?;
    Ok(())
}
