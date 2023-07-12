use super::{query::GithubSearchQuery, schema::CodeSearchResponse};

const GITHUB_API_URL: &'static str = "https://api.github.com";

/// A client for interacting with the GitHub API.
pub struct GithubClient {
    client: reqwest::Client,
    token: String,
}

impl GithubClient {
    /// Create a new GithubClient with the given api token
    pub fn new(token: String) -> Self {
        Self {
            client: reqwest::Client::new(),
            token,
        }
    }

    /// Search code on GitHub with the given query
    pub async fn search_code(
        &self,
        query: &GithubSearchQuery,
    ) -> Result<CodeSearchResponse, reqwest::Error> {
        let compiled_query = query.build();
        let url = format!("{}/search/code?q={compiled_query}", GITHUB_API_URL);
        self.client
            .get(&url)
            .bearer_auth(&self.token)
            .header("accept", "application/vnd.github+json")
            .send()
            .await?
            .json::<CodeSearchResponse>()
            .await
    }
}
