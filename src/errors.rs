use pyo3::prelude::*;

use pyo3::exceptions::PyException;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum SeaSerpentError {
    #[error(transparent)]
    Database(#[from] seaserpent::database::DatabaseError),
    #[error(transparent)]
    Search(#[from] seaserpent::search::SearchError),
    #[error("")]
    Formatting,
}

impl std::convert::From<SeaSerpentError> for PyErr {
    fn from(err: SeaSerpentError) -> PyErr {
        PyException::new_err(err.to_string())
    }
}
