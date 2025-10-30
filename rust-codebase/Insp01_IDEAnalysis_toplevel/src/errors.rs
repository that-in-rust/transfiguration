use thiserror::Error;

pub type Result<T> = std::result::Result<T, ResearchError>;

#[derive(Error, Debug)]
pub enum ResearchError {
    #[error("Database error: {0}")]
    Database(#[from] sqlx::Error),
    
    #[error("HTTP request failed: {0}")]
    Http(#[from] reqwest::Error),
    
    #[error("GitHub API error: {0}")]
    GitHub(#[from] octocrab::Error),
    
    #[error("Serialization error: {0}")]
    Serialization(#[from] serde_json::Error),
    
    #[error("Validation error: {field} - {message}")]
    Validation { field: String, message: String },
    
    #[error("Data quality error: {source} - {issue}")]
    DataQuality { source: String, issue: String },
    
    #[error("Source verification failed: {url} - {reason}")]
    SourceVerification { url: String, reason: String },
    
    #[error("Research bias detected: {bias_type} - {description}")]
    BiasDetection { bias_type: String, description: String },
    
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
}