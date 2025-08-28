use std::fmt;

/// All possible errors that can occur when using the ElevenLabs API
#[derive(Debug)]
pub enum ElevenLabsError {
    /// HTTP request failed (network issues, timeout, etc.)
    RequestError(reqwest::Error),

    /// API returned an error status code
    ApiError { status: u16, message: String },

    /// Failed to parse JSON response
    ParseError(serde_json::Error),

    /// Invalid API key or authentication failed
    AuthenticationError(String),

    /// Rate limit exceeded
    RateLimitError {
        retry_after: Option<u64>, // seconds
        message: String,
    },

    /// Quota exceeded (not enough credits)
    QuotaExceededError(String),

    /// Invalid input parameters
    ValidationError(String),
}

impl fmt::Display for ElevenLabsError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ElevenLabsError::RequestError(e) => write!(f, "Request failed: {}", e),
            ElevenLabsError::ApiError { status, message } => {
                write!(f, "API error ({}): {}", status, message)
            }
            ElevenLabsError::ParseError(e) => write!(f, "Failed to parse response: {}", e),
            ElevenLabsError::AuthenticationError(msg) => {
                write!(f, "Authentication failed: {}", msg)
            }
            ElevenLabsError::RateLimitError {
                retry_after,
                message,
            } => match retry_after {
                Some(seconds) => write!(
                    f,
                    "Rate limit exceeded (retry in {}s): {}",
                    seconds, message
                ),
                None => write!(f, "Rate limit exceeded: {}", message),
            },
            ElevenLabsError::QuotaExceededError(msg) => write!(f, "Quota exceeded: {}", msg),
            ElevenLabsError::ValidationError(msg) => write!(f, "Validation error: {}", msg),
        }
    }
}

impl std::error::Error for ElevenLabsError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            ElevenLabsError::RequestError(e) => Some(e),
            ElevenLabsError::ParseError(e) => Some(e),
            _ => None,
        }
    }
}

impl From<reqwest::Error> for ElevenLabsError {
    fn from(error: reqwest::Error) -> Self {
        // Check if it's a specific HTTP status error
        if let Some(status) = error.status() {
            let status_code = status.as_u16();
            match status_code {
                401 => ElevenLabsError::AuthenticationError("Invalid API key".to_string()),
                429 => {
                    // Try to extract retry-after header if available
                    ElevenLabsError::RateLimitError {
                        retry_after: None, // Could be enhanced to parse Retry-After header
                        message: "Too many requests".to_string(),
                    }
                }
                402 => ElevenLabsError::QuotaExceededError("Insufficient credits".to_string()),
                _ => ElevenLabsError::ApiError {
                    status: status_code,
                    message: error.to_string(),
                },
            }
        } else {
            ElevenLabsError::RequestError(error)
        }
    }
}

impl From<serde_json::Error> for ElevenLabsError {
    fn from(error: serde_json::Error) -> Self {
        ElevenLabsError::ParseError(error)
    }
}
