use pyo3::{pyclass, pymethods, PyAny, PyResult, Python};

use super::{errors::GithubClientError, query::GithubSearchQuery, schema::CodeSearchResponse};

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
        // Not ideal to do this, but clients are light enough for now 
        let other = self.clone();

        pyo3_asyncio::tokio::future_into_py(py, async move {
            other.search_code(&query, per_page, page_number).await.map_err(|e| e.into())
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
        let per_page = per_page.unwrap_or(30);
        let page_number = page_number.unwrap_or(1);
        let compiled_query = query.build();
        let url = format!("{GITHUB_API_URL}/search/code?q={compiled_query}&per_page={per_page}&page={page_number}");
        
        let response = self.client
            .get(&url)
            .bearer_auth(&self.token)
            .header("Accept", "application/vnd.github.text-match+json")
            .header("User-Agent", "ghsrch")
            .send()
            .await
            .map_err(|e| GithubClientError::from(e))?;
    
         response.json::<CodeSearchResponse>()
           .await
           .map_err(|e| GithubClientError::from(e))
    }
}
