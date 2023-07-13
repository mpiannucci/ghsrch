use pyo3::{pyclass, pymethods, PyAny, PyResult, Python};

use super::{errors::GithubClientError, query::GithubSearchQuery, schema::{CodeSearchResponse, CommitSearchResponse}};

const GITHUB_API_URL: &'static str = "https://api.github.com";

/// A client for interacting with the GitHub API.
#[derive(Clone)]
#[pyclass]
pub struct GithubClient {
    client: reqwest::Client,
    token: String,
}

#[pymethods]
impl GithubClient {
    /// Create a new GithubClient with the given api token
    #[new]
    #[pyo3(signature = (token))]
    pub fn new(token: String) -> Self {
        Self {
            client: reqwest::Client::new(),
            token,
        }
    }

    /// Search code on GitHub with the given query
    #[pyo3(name = "search_code")]
    #[pyo3(signature = (query, per_page = None, page_number = None))]
    pub fn search_code0<'py>(&self, py: Python<'py>, query: GithubSearchQuery, per_page: Option<usize>, page_number: Option<usize>) -> PyResult<&'py PyAny> {
        // Not ideal to do this, but clients are light enough for now. This needs
        // to be cloned and moved so the closure will be owned by the GIL, not in the same 
        // lifetime as the client object. Can probably do lifetime stuff here instead but
        // avoiding that complexity for now
        let other = self.clone();

        pyo3_asyncio::tokio::future_into_py(py, async move {
            other.search_code(&query, per_page, page_number).await.map_err(|e| e.into())
        })
    }

    /// Search commits on GitHub with the given query
    #[pyo3(name = "search_commits")]
    #[pyo3(signature = (query, per_page = None, page_number = None))]
    pub fn search_commits0<'py>(&self, py: Python<'py>, query: GithubSearchQuery, per_page: Option<usize>, page_number: Option<usize>) -> PyResult<&'py PyAny> {
        // Not ideal to do this, but clients are light enough for now. This needs
        // to be cloned and moved so the closure will be owned by the GIL, not in the same 
        // lifetime as the client object. Can probably do lifetime stuff here instead but
        // avoiding that complexity for now
        let other = self.clone();

        pyo3_asyncio::tokio::future_into_py(py, async move {
            other.search_commits(&query, per_page, page_number).await.map_err(|e| e.into())
        })
    }
}

impl GithubClient {
    /// Search code on GitHub with the given query
    pub async fn search_code(
        &self,
        query: &GithubSearchQuery,
        per_page: Option<usize>,
        page_number: Option<usize>,
    ) -> Result<CodeSearchResponse, GithubClientError> {
        self.search("code".into(), query, per_page, page_number).await
    }

    /// Search commits on GitHub with the given query
    pub async fn search_commits(
        &self,
        query: &GithubSearchQuery,
        per_page: Option<usize>,
        page_number: Option<usize>,
    ) -> Result<CommitSearchResponse, GithubClientError> {
        self.search("commits".into(), query, per_page, page_number).await
    }

    /// Search the given path on GitHub with the given query
    /// returning the reponse object specifed by the github rest API
    async fn search<T>(
        &self,
        path: String,
        query: &GithubSearchQuery,
        per_page: Option<usize>,
        page_number: Option<usize>,
    ) -> Result<T, GithubClientError> where T: serde::de::DeserializeOwned {
        // API by default returns 30 results per page, so we'll default to that
        let per_page = per_page.unwrap_or(30);
        // API by default returns the first page
        let page_number = page_number.unwrap_or(1);
        let compiled_query = query.build();
        let url = format!("{GITHUB_API_URL}/search/{path}?q={compiled_query}&per_page={per_page}&page={page_number}");
        
        let response = self.client
            .get(&url)
            .bearer_auth(&self.token)
            .header("Accept", "application/vnd.github.text-match+json")
            .header("User-Agent", "ghsrch")
            .send()
            .await
            .map_err(|e| GithubClientError::from(e))?;
    
         response.json::<T>()
           .await
           .map_err(|e| GithubClientError::from(e))
    }
}
