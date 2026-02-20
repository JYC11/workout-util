use crate::client::run_app;
use std::error::Error;

pub mod client;
mod db;
pub mod enums;
pub mod exercise;
mod timer;
pub mod workout;
pub mod workout_log;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    run_app().await?;
    Ok(())
}
