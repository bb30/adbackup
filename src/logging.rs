use chrono;
use fern;
use log;

pub fn setup_logging(verbosity: u64) -> Result<(), fern::InitError> {
    let mut base_config = fern::Dispatch::new();

    base_config = match verbosity {
        0 => {
            // Let's say we depend on something which whose "info" level messages are too verbose
            // to include in end-user output. If we don't need them, let's not include them.
            base_config
                .level(log::LogLevelFilter::Info)
                .level_for("overly-verbose-target", log::LogLevelFilter::Warn)
        }
        1 => base_config
            .level(log::LogLevelFilter::Debug)
            .level_for("overly-verbose-target", log::LogLevelFilter::Info),
        2 => base_config.level(log::LogLevelFilter::Debug),
        _3_or_more => base_config.level(log::LogLevelFilter::Trace),
    };

    // Separate file config so we can include time/date in file logs
    let file_config = fern::Dispatch::new()
        .format(|out, message, record| {
            out.finish(format_args!(
                "{}[{}][{}] {}",
                chrono::Local::now().format("[%Y-%m-%d][%H:%M:%S]"),
                record.target(),
                record.level(),
                message
            ))
        })
        .chain(fern::log_file("adbackup.log")?);

    base_config.chain(file_config).apply()?;

    Ok(())
}
