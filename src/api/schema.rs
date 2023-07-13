use pyo3::pyclass;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
#[pyclass]
pub struct Author {
    #[pyo3(get)]
    pub name: String,
    #[pyo3(get)]
    pub email: String,
    #[pyo3(get)]
    pub date: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[pyclass]
pub struct User {
    #[pyo3(get)]
    pub name: Option<String>,
    #[pyo3(get)]
    pub email: Option<String>,
    #[pyo3(get)]
    pub login: String,
    #[pyo3(get)]
    pub id: u64,
    #[pyo3(get)]
    pub node_id: String,
    #[pyo3(get)]
    pub avatar_url: String,
    #[pyo3(get)]
    pub gravatar_id: String,
    #[pyo3(get)]
    pub url: String,
    #[pyo3(get)]
    pub html_url: String,
    #[pyo3(get)]
    pub followers_url: String,
    #[pyo3(get)]
    pub following_url: String,
    #[pyo3(get)]
    pub gists_url: String,
    #[pyo3(get)]
    pub starred_url: String,
    #[pyo3(get)]
    pub subscriptions_url: String,
    #[pyo3(get)]
    pub organizations_url: String,
    #[pyo3(get)]
    pub repos_url: String,
    #[pyo3(get)]
    pub events_url: String,
    #[pyo3(get)]
    pub received_events_url: String,
    #[serde(rename = "type")]
    #[pyo3(get)]
    pub user_type: String,
    #[pyo3(get)]
    pub site_admin: bool,
    #[pyo3(get)]
    pub starred_at: Option<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[pyclass]
pub struct Repository {
    #[pyo3(get)]
    pub id: u64,
    #[pyo3(get)]
    pub node_id: String,
    #[pyo3(get)]
    pub name: String,
    #[pyo3(get)]
    pub full_name: String,
    #[pyo3(get)]
    pub owner: User,
    #[pyo3(get)]
    pub private: bool,
    #[pyo3(get)]
    pub html_url: String,
    #[pyo3(get)]
    pub description: Option<String>,
    #[pyo3(get)]
    pub fork: bool,
    #[pyo3(get)]
    pub url: String,
    #[pyo3(get)]
    pub archive_url: String,
    #[pyo3(get)]
    pub assignees_url: String,
    #[pyo3(get)]
    pub blobs_url: String,
    #[pyo3(get)]
    pub branches_url: String,
    #[pyo3(get)]
    pub collaborators_url: String,
    #[pyo3(get)]
    pub comments_url: String,
    #[pyo3(get)]
    pub commits_url: String,
    #[pyo3(get)]
    pub compare_url: String,
    #[pyo3(get)]
    pub contents_url: String,
    #[pyo3(get)]
    pub contributors_url: String,
    #[pyo3(get)]
    pub deployments_url: String,
    #[pyo3(get)]
    pub downloads_url: String,
    #[pyo3(get)]
    pub events_url: String,
    #[pyo3(get)]
    pub forks_url: String,
    #[pyo3(get)]
    pub git_commits_url: String,
    #[pyo3(get)]
    pub git_refs_url: String,
    #[pyo3(get)]
    pub git_tags_url: String,
    #[pyo3(get)]
    pub git_url: Option<String>,
    #[pyo3(get)]
    pub issue_comment_url: String,
    #[pyo3(get)]
    pub issue_events_url: String,
    #[pyo3(get)]
    pub issues_url: String,
    #[pyo3(get)]
    pub labels_url: String,
    #[pyo3(get)]
    pub languages_url: String,
    #[pyo3(get)]
    pub merges_url: String,
    #[pyo3(get)]
    pub pulls_url: String,
    #[pyo3(get)]
    pub releases_url: String,
    #[pyo3(get)]
    pub ssh_url: Option<String>,
    #[pyo3(get)]
    pub stargazers_url: String,
    #[pyo3(get)]
    pub statuses_url: String,
    #[pyo3(get)]
    pub subscribers_url: String,
    #[pyo3(get)]
    pub subscription_url: String,
    #[pyo3(get)]
    pub tags_url: String,
    #[pyo3(get)]
    pub teams_url: String,
    #[pyo3(get)]
    pub clone_url: Option<String>,
    #[pyo3(get)]
    pub homepage: Option<String>,
    #[pyo3(get)]
    pub language: Option<String>,
    #[pyo3(get)]
    pub forks_count: Option<u64>,
    #[pyo3(get)]
    pub stargazers_count: Option<u64>,
    #[pyo3(get)]
    pub watchers_count: Option<u64>,
    #[pyo3(get)]
    pub size: Option<f64>,
    #[pyo3(get)]
    pub default_branch: Option<String>,
    #[pyo3(get)]
    pub open_issues_count: Option<f64>,
    #[pyo3(get)]
    pub topics: Option<Vec<String>>,
    #[pyo3(get)]
    pub has_issues: Option<bool>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[pyclass]
pub struct Commit {
    #[pyo3(get)]
    pub message: String,
    #[pyo3(get)]
    pub url: String,
    #[pyo3(get)]
    pub comment_count: u64,
    #[pyo3(get)]
    pub author: Author,
    #[pyo3(get)]
    pub committer: Option<Author>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[pyclass]
pub struct CodeSearchTextMatchHighlights {
    #[pyo3(get)]
    pub text: String,
    #[pyo3(get)]
    pub indices: Vec<usize>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[pyclass]
pub struct CodeSearchTextMatch {
    #[pyo3(get)]
    pub object_url: String,
    #[pyo3(get)]
    pub object_type: String,
    #[pyo3(get)]
    pub property: String,
    #[pyo3(get)]
    pub fragment: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[pyclass]
pub struct CodeSearchResult {
    #[pyo3(get)]
    pub name: String,
    #[pyo3(get)]
    pub path: String,
    #[pyo3(get)]
    pub sha: String,
    #[pyo3(get)]
    pub url: String,
    #[pyo3(get)]
    pub git_url: String,
    #[pyo3(get)]
    pub html_url: String,
    #[pyo3(get)]
    pub repository: Repository,
    #[pyo3(get)]
    pub score: f64,
    #[pyo3(get)]
    pub file_size: Option<usize>,
    #[pyo3(get)]
    pub language: Option<String>,
    #[pyo3(get)]
    pub last_modified_at: Option<String>,
    #[pyo3(get)]
    pub line_numbers: Option<Vec<usize>>,
    #[pyo3(get)]
    pub text_matches: Vec<CodeSearchTextMatch>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[pyclass]
pub struct CodeSearchResponse {
    #[pyo3(get)]
    pub total_count: u64,
    #[pyo3(get)]
    pub incomplete_results: bool,
    #[pyo3(get)]
    pub items: Vec<CodeSearchResult>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[pyclass]
pub struct CommitSearchResult {
    #[pyo3(get)]
    pub url: String,
    #[pyo3(get)]
    pub sha: String,
    #[pyo3(get)]
    pub html_url: String,
    #[pyo3(get)]
    pub comments_url: String,
    #[pyo3(get)]
    pub commit: Commit,
    #[pyo3(get)]
    pub author: Option<User>,
    #[pyo3(get)]
    pub committer: Option<Author>,
    #[pyo3(get)]
    pub repository: Repository,
    #[pyo3(get)]
    pub score: f64,
    #[pyo3(get)]
    pub node_id: String,
    #[pyo3(get)]
    pub text_matches: Vec<CodeSearchTextMatch>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[pyclass]
pub struct CommitSearchResponse {
    #[pyo3(get)]
    pub total_count: u64,
    #[pyo3(get)]
    pub incomplete_results: bool,
    #[pyo3(get)]
    pub items: Vec<CommitSearchResult>,
}