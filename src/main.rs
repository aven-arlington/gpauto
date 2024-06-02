use gpauto::cli;
use gpauto::{Application, ApplicationBuilder};
use tracing::{debug, error, info, warn};
use tracing_subscriber::EnvFilter;

pub fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Argument parsing
    let cli = cli::parse_args();

    // Global tracing events based on RUST_LOG environment variable
    // E.x. > RUST_LOG=info cargo run
    // E.x. > export RUST_LOG=Info
    let subscriber = tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .pretty()
        .without_time()
        .finish();
    tracing::subscriber::set_global_default(subscriber)?;

    // Demo functionality
    debug!("Debug");
    info!("Info");
    warn!("Warn");
    error!("Error");

    // Initialize the application
    let app: Application = ApplicationBuilder::new().set_cli_arguments(cli).build();

    // Run the application
    app.run();
    Ok(())
}
