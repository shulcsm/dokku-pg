extern crate log;

use log::{LogLevel, LogLevelFilter, LogRecord, SetLoggerError, LogMetadata};

pub struct DokkuLogger;

macro_rules! print_err {
    ($($arg:tt)*) => (
        {
            use std::io::prelude::*;
            if let Err(..) = write!(&mut ::std::io::stderr(), "{}\n", format_args!($($arg)*)) {
                panic!("Failed to write to stderr.\
                        \nOriginal error out")
            }
        })
}


impl log::Log for DokkuLogger {
    fn enabled(&self, metadata: &LogMetadata) -> bool {
        metadata.level() <= LogLevel::Info
    }

    fn log(&self, record: &LogRecord) {
        if self.enabled(record.metadata()) {
            if record.level() <= LogLevel::Error {
                print_err!("{} - {}", record.level(), record.args());
            } else {
                println!("{} - {}", record.level(), record.args());
            }
        }
    }
}

pub fn init() -> Result<(), SetLoggerError> {
    log::set_logger(|max_log_level| {
        max_log_level.set(LogLevelFilter::Info);
        Box::new(DokkuLogger)
    })
}
