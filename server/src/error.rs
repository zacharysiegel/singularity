use std::backtrace::{Backtrace, BacktraceStatus};
use std::error::Error;
use std::fmt;
use std::fmt::{Display, Formatter};

/// Should be initialized lazily (e.g. [Option::ok_or_else]) for captured backtraces to make sense.
#[derive(Debug)]
pub struct AppError {
    pub message: String,
    pub sub_error: Option<Box<dyn Error>>,
    pub backtrace: Backtrace,
}

impl AppError {
    const DEFAULT_MESSAGE: &'static str = "unspecified";

    pub fn new(message: &str) -> AppError {
        Self::_new(message, None)
    }

    pub fn from_error(message: &str, error: Box<dyn Error>) -> AppError {
        Self::_new(message, Some(error))
    }

    pub fn from_error_default(error: Box<dyn Error>) -> AppError {
        Self::_new(Self::DEFAULT_MESSAGE, Some(error))
    }

    fn _new(message: &str, error: Option<Box<dyn Error>>) -> AppError {
        let backtrace: Backtrace = Backtrace::force_capture();
        let shop_error = AppError {
            message: format!("Error: {}", message),
            sub_error: error,
            backtrace,
        };
        shop_error
    }
}

impl Display for AppError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "ShopError [{}]", self.message)?;
        if let Some(sub_error) = &self.sub_error {
            write!(f, "\n[{}]", sub_error)?;
        }
        match self.backtrace.status() {
            BacktraceStatus::Unsupported | BacktraceStatus::Disabled => Ok(()),
            BacktraceStatus::Captured => write!(f, "\n{}", self.backtrace),
            _ => Ok(()),
        }
    }
}

impl Error for AppError {}

impl Default for AppError {
    fn default() -> Self {
        Self::new(Self::DEFAULT_MESSAGE)
    }
}

impl From<std::io::Error> for AppError {
    fn from(err: std::io::Error) -> Self {
        AppError::from_error(&err.to_string(), Box::new(err))
    }
}

