use strum::Display;
use tracing_subscriber::{
  filter::LevelFilter
};


#[derive(Default, Display)]
#[strum(serialize_all = "snake_case")]
pub enum Level {
  Trace,
  Debug,
  #[default]
  Info,
  Warn,
  Error,
}

impl From<Level> for LevelFilter {
  fn from(value: Level) -> Self {
    match value {
      Level::Trace => LevelFilter::TRACE,
      Level::Debug => LevelFilter::DEBUG,
      Level::Info => LevelFilter::INFO,
      Level::Warn => LevelFilter::WARN,
      Level::Error => LevelFilter::ERROR,
    }
  }
}

// impl From<Level> for tracing::Level {
//   fn from(value: Level) -> Self {
//     match value {
//       Level::Trace => tracing::Level::TRACE,
//       Level::Debug => tracing::Level::DEBUG,
//       Level::Info  => tracing::Level::INFO,
//       Level::Warn  => tracing::Level::WARN,
//       Level::Error => tracing::Level::ERROR,
//     }
//   }
// }

pub fn init_debug(_user_logging_level: Option<Level>) {
  #[cfg(debug_assertions)]
  init(_user_logging_level);
}

pub fn init(user_logging_level: Option<Level>) {
  tracing_subscriber::fmt()
    .with_max_level(match user_logging_level {
      None => LevelFilter::OFF,
      Some(l) => l.into()
    })
    .with_thread_names(true)
    .init();
}