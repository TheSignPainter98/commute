#[derive(thiserror::Error, Debug)]
pub(crate) enum Error {
    #[error("io error: {0}")]
    IOError(#[from] std::io::Error),

    #[error("json conversion error: {0}")]
    JSONError(#[from] serde_json::Error),

    #[error("{0}: {}", .0.root_cause())]
    AnyHowError(#[from] anyhow::Error),
}
