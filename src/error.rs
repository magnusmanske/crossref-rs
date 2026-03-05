use crate::query::ResourceComponent;
use crate::response::MessageType;
use std::result;

/// A type alias for handling errors throughout crossref.
pub type Result<T> = result::Result<T, Error>;

/// all different error types this crate uses
#[derive(Debug, thiserror::Error)]
pub enum Error {
    /// if an invalid type was requested
    #[error("invalid type name: {name}")]
    InvalidTypeName {
        /// the name of the invalid type
        name: String,
    },

    /// if there is a mismatch between the expected return type of the crossref api and this rust client
    #[error("expected response item of type {expected} but got {got}")]
    UnexpectedItem {
        /// the expected type
        expected: MessageType,
        /// the received type
        got: MessageType,
    },
    /// a config error
    #[error("{msg}")]
    Config {
        /// the notification
        msg: String,
    },

    /// an error that occurred while operating with [reqwest]
    #[error("{0}")]
    ReqWest(#[from] reqwest::Error),
    /// When no message was found but expected
    #[error("No message found but expected message of type `{expected}`")]
    MissingMessage {
        /// the expected type
        expected: MessageType,
    },
    /// When crossref could not find anything
    #[error("Nothing was found for resource `{resource}`")]
    ResourceNotFound {
        /// the resource that was not found
        resource: Box<ResourceComponent>,
    },
    /// if a error in serde occurred
    #[error("invalid serde: {0}")]
    Serde(#[from] serde_json::Error),
}
