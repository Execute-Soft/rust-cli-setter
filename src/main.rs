mod app;
mod cli;
mod commands;
mod error;

use error::AppError;

#[tokio::main]
async fn main() -> Result<(), AppError> {
    app::run().await
}
