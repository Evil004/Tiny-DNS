use std::error::Error;
use std::io::Error as IoError;
use std::fmt::Display;
use std::net::{IpAddr};

#[derive(Debug)]
pub enum ConfigError {
    NoResolverEspecified,
    NoPortSpecified,
    FailedToCreateServer,
}

impl Error for ConfigError {}

impl Display for ConfigError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let message = match self {
            ConfigError::NoResolverEspecified => "No resolver specified",
            ConfigError::NoPortSpecified => "No port specified",
            ConfigError::FailedToCreateServer => "Error creating server"
        };

        write!(f, "{}", message)
    }
}

#[derive(Debug)]
pub enum ServerError {
    FailedToBindSocket,
}

impl Error for ServerError {}

impl Display for ServerError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let message = match self {
            ServerError::FailedToBindSocket => "Failed to bind socket, maybe the port is already in use or you don't have permission to bind it.",
        };

        write!(f, "{}", message)
    }
}


#[derive(Debug)]
pub enum QueryError {
    FailedToSerializeQuery,
    FailetToResolveQuery,
    FailedToDeserializeResponse,
    FailedToSerializeResponse,
}

impl Error for QueryError {}

impl Display for QueryError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let message = match self {
            QueryError::FailedToSerializeQuery => "Failed to serialize query",
            QueryError::FailedToDeserializeResponse => "Failed to deserialize response",
            QueryError::FailedToSerializeResponse => "Failed to serialize response",
            QueryError::FailetToResolveQuery => "Failed to resolve query"
        };

        write!(f, "{}", message)
    }
}

#[derive(Debug)]
pub enum LookupError {
    FailedToBindSocket(IoError),
    FailedToConnectSocket {
        ip: IpAddr,
        port: u16,
    },
    FailedToSetReadTimeout,
    FailedToSendQuery,
    FailedToReceiveResponse(IoError),
    Timeout,
}

impl Error for LookupError {}

impl Display for LookupError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let message = match self {
            LookupError::FailedToBindSocket(e) => format!("Failed to bind socket: {}", e),
            LookupError::FailedToConnectSocket { ip, port } => format!("Failed to connect to socket {}:{}", ip, port),
            LookupError::FailedToSetReadTimeout => String::from("Failed to set read timeout"),
            LookupError::FailedToSendQuery => String::from("Failed to send query"),
            LookupError::FailedToReceiveResponse(e) => format!("Failed to receive response: {}", e),
            LookupError::Timeout => String::from("Timeout, no response received")
        };

        write!(f, "{}", message)
    }
}

#[derive(Debug)]
pub enum DeserializeError {
    InvalidHeader,
    InvalidQuestion,
    InvalidAnswer,
    InvalidAuthority,
    InvalidAdditional,
    InvalidRecord,
}

impl Error for DeserializeError {}

impl Display for DeserializeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let message = match self {
            DeserializeError::InvalidHeader => "Failed to deserialize header",
            DeserializeError::InvalidQuestion => "Failed to deserialize question",
            DeserializeError::InvalidAnswer => "Failed to deserialize answer",
            DeserializeError::InvalidAuthority => "Failed to deserialize authority",
            DeserializeError::InvalidAdditional => "Failed to deserialize additional",
            DeserializeError::InvalidRecord => "Failed to deserialize record"
        };

        write!(f, "{}", message)
    }
}

#[derive(Debug)]
pub enum SerializeError {
    InvalidHeader,
    InvalidQuestion,
    InvalidAnswer,
    InvalidAuthority,
    InvalidAdditional,
    InvalidRecord,
}

impl Error for SerializeError {}

impl Display for SerializeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let message = match self {
            SerializeError::InvalidHeader => "Failed to serialize header",
            SerializeError::InvalidQuestion => "Failed to serialize question",
            SerializeError::InvalidAnswer => "Failed to serialize answer",
            SerializeError::InvalidAuthority => "Failed to serialize authority",
            SerializeError::InvalidAdditional => "Failed to serialize additional",
            SerializeError::InvalidRecord => "Failed to serialize record"
        };

        write!(f, "{}", message)
    }
}