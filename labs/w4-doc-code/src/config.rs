//! This module contains the configuration options for the application.
//! # Examples:
//! ```
//! use cli_utils::config::Logging;
//! let config = Logging::new();
//! ```
//! 
pub enum LogLevel {
    Debug,
    Info,
    Warn,
    Error,
}

/// Enum that represents the output of the log.
/// # Examples:
/// ```
/// use cli_utils::config::LogOutput;
/// let output = LogOutput::Stdout;
/// ```
pub enum LogOutput {
    Stdout,
    Stderr,
    File(String),
}

/// This struct contains configuration options for the application.
/// # Examples:
/// ```
/// use cli_utils::config::Logging;
/// let config = Logging::new();
/// ```
/// 
/// Creating a new instance of the Logging struct:
/// ```
/// use cli_utils::config::{Logging, LogLevel, LogOutput};
/// let config = Logging{ enabled: true, level: LogLevel::Info, destination: LogOutput::Stdout };
/// ```
pub struct Logging {
    pub enabled: bool,
    pub level: LogLevel,
    pub destination: LogOutput,   
}

/// Implementation of the Logging struct.
/// # Examples:
/// ```
/// use cli_utils::config::Logging;
/// let config = Logging::new();
/// ```
impl Logging {
    pub fn new() -> Self {
        Self {
            enabled: false,
            level: LogLevel::Info,
            destination: LogOutput::Stdout,
        }
    }
}