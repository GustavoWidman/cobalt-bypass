use colog::format::CologStyle;
use env_logger::Builder;
use log::{Level, LevelFilter};

struct CustomLevelTokens;

impl CologStyle for CustomLevelTokens {
    fn level_token(&self, level: &Level) -> &str {
        match *level {
            Level::Error => "ERR",
            Level::Warn => "WRN",
            Level::Info => "INF",
            Level::Debug => "DBG",
            Level::Trace => "TRC",
        }
    }
}

pub struct Logger;

impl Logger {
    pub fn init(level: Option<LevelFilter>) {
        Builder::new()
            .filter(None, level.unwrap_or(LevelFilter::Info))
            .target(env_logger::Target::Stdout)
            .format(colog::formatter(CustomLevelTokens))
            .write_style(env_logger::WriteStyle::Always)
            .init();
    }
}
