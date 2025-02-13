use dropshot::ErrorStatusCode;
use dropshot::HttpError;
use dropshot::HttpResponseError;
use schemars::JsonSchema;
use serde::Serialize;

#[derive(Debug, Serialize, JsonSchema)]
pub enum ResourceKind {
    Instance,
}

#[derive(Debug, Serialize, JsonSchema, thiserror::Error)]
pub enum ConsoleError {
    #[error("no {:?} found", .kind)]
    ResourceNotFound { kind: ResourceKind },

    #[error("{internal_message}")]
    Other {
        message: String,
        error_code: Option<String>,

        #[serde(skip)]
        internal_message: String,
        #[serde(skip)]
        status: ErrorStatusCode,
    },
}

impl HttpResponseError for ConsoleError {
    fn status_code(&self) -> dropshot::ErrorStatusCode {
        match self {
            ConsoleError::ResourceNotFound { .. } => dropshot::ErrorStatusCode::NOT_FOUND,

            ConsoleError::Other { status, .. } => *status,
        }
    }
}

impl From<HttpError> for ConsoleError {
    fn from(error: HttpError) -> Self {
        ConsoleError::Other {
            message: error.external_message,
            internal_message: error.internal_message,
            status: error.status_code,
            error_code: error.error_code,
        }
    }
}
