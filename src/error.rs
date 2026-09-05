use thiserror::Error;

#[derive(Debug,Error)]
pub enum TaskError {
    #[error("failed to read the file:{0}")]
    FileRead(#[from] std::io::Error),

    #[error("failed to parse the json:{0}")]
    JsonParse(#[from] serde_json::Error),

    #[error("task '{0}' didnt found")]
    NotFound(String)
}