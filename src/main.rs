use log::{debug, info, warn};
use std::time::SystemTime;
use fern::colors::{Color, ColoredLevelConfig, WithFgColor};
use log::Level::{Info};

fn setup_logger() -> Result<(), fern::InitError> {
    let logLevelColors = ColoredLevelConfig::new()
        .info(Color::Green)
        .warn(Color::BrightYellow)
        .error(Color::Red)
        .debug(Color::Black)
        .trace(Color::White);

    let messageColors = ColoredLevelConfig::new()
        .info(Color::Blue);

    fern::Dispatch::new()
        .format(move |out, message, record| {
            out.finish(format_args!(
                "[{}] [{}] {} {}",
                humantime::format_rfc3339_seconds(SystemTime::now()),
                logLevelColors.color(record.level()),
                messageColors.color(Info),
                message
            ))
        })
        .level(log::LevelFilter::Debug)
        .chain(std::io::stdout())
        .chain(fern::log_file("output.log")?)
        .apply()?;
    Ok(())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    setup_logger()?;


    info!("Hello, world!");
    warn!("Warning!");
    debug!("Now exiting.");

    Ok(())
}