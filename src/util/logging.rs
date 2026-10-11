use color_eyre::Result;
use std::fs::File;
use std::path::Path;
use tracing_subscriber::EnvFilter;

/// Sets up file-based logging for the game. Wipes any previous log file
/// so each run starts fresh. Call this once, before entering the
/// terminal's alternate screen.
pub fn init() -> Result<()> {
    let log_dir = Path::new("logs");
    std::fs::create_dir_all(log_dir)?;

    let log_path = log_dir.join("debug.log");

    // Truncate/create fresh each run (File::create truncates if it exists)
    let log_file = File::create(&log_path)?;

    let filter = EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("debug"));

    tracing_subscriber::fmt()
        .with_writer(log_file)
        .with_ansi(true) // no color codes in the plain file
        .without_time()
        .with_target(true) // show module path per line, handy for a TUI app
        .with_line_number(true)
        .with_level(true)
        .with_env_filter(filter)
        .init();

    Ok(())
}

#[macro_export]
macro_rules! log {
    ($($arg:tt)*) => { tracing::info!($($arg)*) };
}
