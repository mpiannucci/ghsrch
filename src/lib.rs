mod api;

use api::{
    client::GithubClient,
    query::GithubSearchQuery,
    schema::{
        Author, CodeSearchResponse, CodeSearchResult, CodeSearchTextMatch, Commit,
        CommitSearchResponse, CommitSearchResult, Repository, User,
    },
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
    m.add_class::<Author>()?;
    m.add_class::<Commit>()?;
    m.add_class::<CommitSearchResult>()?;
    m.add_class::<CommitSearchResponse>()?;
    Ok(())
}
