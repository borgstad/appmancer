
use log4rs;
use std::fs::OpenOptions;
use std::io::Write;

pub fn init() {
    clear_log_file("log/my.log").expect("Failed to clear log file");
    log4rs::init_file("logging_config.yaml", Default::default()).unwrap();
    // trace!("detailed tracing info");
    // debug!("debug info");
    // info!("relevant general info");
    // warn!("warning this program doesn't do much");
    // error!("error message here");
}

fn clear_log_file(log_file_path: &str) -> std::io::Result<()> {
    let file = OpenOptions::new()
        .write(true)
        .truncate(true)
        .open(log_file_path)?;
    file.sync_all()?;
    Ok(())
}
