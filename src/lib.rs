mod api;

use api::{
    client::GithubClient,
    schema::{CodeSearchResponse, CodeSearchResult, CodeSearchTextMatch, Repository, User}, query::GithubSearchQuery,
};
use pyo3::prelude::*;

/// A Python module implemented in Rust.
#[pymodule]
fn ghsrch(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<GithubSearchQuery>()?;
    m.add_class::<GithubClient>()?;
    m.add_class::<CodeSearchResponse>()?;
    m.add_class::<CodeSearchResult>()?;
    m.add_class::<CodeSearchTextMatch>()?;
    m.add_class::<Repository>()?;
    m.add_class::<User>()?;
    Ok(())
}
