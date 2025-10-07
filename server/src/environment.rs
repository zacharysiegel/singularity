use std::env;
use std::fmt::Display;
use std::ops::Deref;
use std::string::ToString;
use std::sync::LazyLock;

use shared::error::AppError;

static RUNTIME_ENVIRONMENT_DEFAULT: LazyLock<RuntimeEnvironment> =
    LazyLock::new(|| RuntimeEnvironment::from_env().unwrap_or(RuntimeEnvironment::Local));
#[allow(unused)]
static CARGO_MANIFEST_DIR: LazyLock<String> =
    LazyLock::new(|| env::var("CARGO_MANIFEST_DIR").unwrap_or("/dev/null".to_string()));

pub const VOLATILE_DIRECTORY_NAME: &str = "volatile";
pub const IMAGES_DIRECTORY_NAME: &str = "images";

#[derive(Debug, PartialEq, Clone)]
pub enum RuntimeEnvironment {
    Local = 0,
    Stage,
    Production,
}

impl RuntimeEnvironment {
    pub fn from_env() -> Result<RuntimeEnvironment, AppError> {
        RuntimeEnvironment::try_from(
            env::var("RUNTIME_ENVIRONMENT").unwrap_or(String::from("local")),
        )
    }

    pub fn get_address(&self) -> &'static str {
        match self {
            RuntimeEnvironment::Local => "0.0.0.0:1443",
            RuntimeEnvironment::Stage => "0.0.0.0:1443", // todo
            RuntimeEnvironment::Production => "0.0.0.0:1443", // todo
        }
    }
}

impl Default for RuntimeEnvironment {
    fn default() -> RuntimeEnvironment {
        RUNTIME_ENVIRONMENT_DEFAULT.deref().clone()
    }
}

impl TryFrom<String> for RuntimeEnvironment {
    type Error = AppError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        match value.as_str() {
            "local" => Ok(Self::Local),
            "stage" => Ok(Self::Stage),
            "production" => Ok(Self::Production),
            _ => Err(AppError::new(&format!(
                "Error parsing runtime environment [{}]",
                value
            ))),
        }
    }
}

impl Display for RuntimeEnvironment {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str = match self {
            RuntimeEnvironment::Local => String::from("local"),
            RuntimeEnvironment::Stage => String::from("stage"),
            RuntimeEnvironment::Production => String::from("production"),
        };
        write!(f, "{}", str)
    }
}

pub fn load_env() -> Result<(), Box<dyn std::error::Error>> {
    dotenvy::dotenv()?;
    Ok(())
}
