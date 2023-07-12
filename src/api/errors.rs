use pyo3::{exceptions::PyOSError, PyErr};

pub enum GithubClientError {
    UnknownError(String),
    ReqwestError(reqwest::Error),
}

impl From<reqwest::Error> for GithubClientError {
    fn from(error: reqwest::Error) -> Self {
        Self::ReqwestError(error)
    }
}

impl From<GithubClientError> for PyErr {
    fn from(error: GithubClientError) -> Self {
        match error {
            GithubClientError::ReqwestError(error) => PyOSError::new_err(error.to_string()),
            GithubClientError::UnknownError(error) => PyOSError::new_err(error),
        }
    }
}
