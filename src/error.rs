use std::fmt::Display;

#[derive(thiserror::Error, Debug)]
pub(crate) enum Error {
    #[error("io error: {0}")]
    IOError(#[from] std::io::Error),

    #[error("json conversion error: {0}")]
    JSONError(#[from] serde_json::Error),

    #[error("{0}: {}", .0.root_cause())]
    AnyHowError(#[from] anyhow::Error),

    #[error("glib: {0}")]
    GLibBoolError(#[from] gio::glib::error::BoolError),

    #[error("failed to execute `{name}`: {reason}")]
    ChildProcessError {
        name: String,
        reason: ChildProcessExit,
    },
}

#[derive(Debug)]
pub(crate) enum ChildProcessExit {
    Exit(i32),
    Signal,
}

impl From<Option<i32>> for ChildProcessExit {
    fn from(value: Option<i32>) -> Self {
        match value {
            Some(c) => Self::Exit(c),
            None => Self::Signal,
        }
    }
}

impl Display for ChildProcessExit {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Exit(c) => write!(f, "process exited with status code: {c}"),
            Self::Signal => write!(f, "process terminated by signal"),
        }
    }
}