use crate::println;

struct LevelColor {
    error: u8,
    warn: u8,
    info: u8,
    debug: u8,
    trace: u8,
}

const LEVEL_COLOR: LevelColor = LevelColor {
    error: 31, // red
    warn: 93, // bright yellow
    info: 34, // blue
    debug: 32, // green
    trace: 90, // bright black
};

struct Logger;

impl log::Log for Logger {
    fn enabled(&self, _metadata: &log::Metadata) -> bool {
        true
    }

    fn log(&self, record: &log::Record) {
        if !self.enabled(record.metadata()) {
            return;
        }

        let color = match record.level() {
            log::Level::Error => LEVEL_COLOR.error,
            log::Level::Warn => LEVEL_COLOR.warn,
            log::Level::Info => LEVEL_COLOR.info,
            log::Level::Debug => LEVEL_COLOR.debug,
            log::Level::Trace => LEVEL_COLOR.trace
        };

        println!(
            "\u{1B}[{}m[{:>5}] {}\u{1B}[0m",
            color,
            record.level(),
            record.args(),
        );
    }

    fn flush(&self) {
        
    }
}

static LOGGER: Logger = Logger;

pub fn init() {
    log::set_logger(&LOGGER).unwrap();

    let max_level = match option_env!("LOG") {
        Some("error") => log::LevelFilter::Error,
        Some("warn") => log::LevelFilter::Warn,
        Some("info") => log::LevelFilter::Info,
        Some("debug") => log::LevelFilter::Debug,
        Some("trace") => log::LevelFilter::Trace,
        _ => log::LevelFilter::Off, 
    };
    log::set_max_level(max_level);
 
    println!("init logger, max level is {}", max_level);
}

