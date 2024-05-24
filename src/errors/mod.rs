use std::error::Error;
use std::fmt::Display;

#[derive(Debug)]
pub enum ConfigError{
    NoResolverEspecified,
    FailedToCreateServer
}

impl Error for ConfigError{
}
impl Display for ConfigError{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let message = match self {
            ConfigError::NoResolverEspecified => "No resolver specified",
            ConfigError::FailedToCreateServer => "Error creating server"
        };
        
        write!(f, "{}", message)
    }
}