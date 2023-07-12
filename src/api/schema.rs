use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct User {
    pub name: String,
    pub email: String,
    pub login: String,
    pub id: u64,
    pub node_id: String,
    pub avatar_url: String,
    pub gravatar_id: String,
    pub url: String,
    pub html_url: String,
    pub followers_url: String,
    pub following_url: String,
    pub gists_url: String,
    pub starred_url: String,
    pub subscriptions_url: String,
    pub organizations_url: String,
    pub repos_url: String,
    pub events_url: String,
    pub received_events_url: String,
    #[serde(rename = "type")]
    pub user_type: String,
    pub site_admin: bool,
    pub starred_at: Option<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Repository {
    pub id: u64,
    pub node_id: String,
    pub name: String,
    pub full_name: String,
    pub owner: User,
    pub private: bool,
    pub html_url: String,
    pub description: Option<String>,
    pub fork: bool,
    pub url: String,
    pub archive_url: String,
    pub assignees_url: String,
    pub blobs_url: String,
    pub branches_url: String,
    pub collaborators_url: String,
    pub comments_url: String,
    pub commits_url: String,
    pub compare_url: String,
    pub contents_url: String,
    pub contributors_url: String,
    pub deployments_url: String,
    pub downloads_url: String,
    pub events_url: String,
    pub forks_url: String,
    pub git_commits_url: String,
    pub git_refs_url: String,
    pub git_tags_url: String,
    pub git_url: String,
    pub issue_comment_url: String,
    pub issue_events_url: String,
    pub issues_url: String,
    pub labels_url: String,
    pub languages_url: String,
    pub merges_url: String,
    pub pulls_url: String,
    pub releases_url: String,
    pub ssh_url: String,
    pub stargazers_url: String,
    pub statuses_url: String,
    pub subscribers_url: String,
    pub subscription_url: String,
    pub tags_url: String,
    pub teams_url: String,
    pub clone_url: String,
    pub homepage: Option<String>,
    pub language: Option<String>,
    pub forks_count: u64,
    pub stargazers_count: u64,
    pub watchers_count: u64,
    pub size: u64,
    pub default_branch: String,
    pub open_issues_count: u64,
    pub topics: Vec<String>,
    pub has_issues: bool,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CodeSearchTextMatchHighlights {
    pub text: String,
    pub indices: Vec<usize>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CodeSearchTextMatch {
    pub object_url: String,
    pub object_type: String,
    pub property: String,
    pub fragment: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CodeSearchResult {
    pub name: String,
    pub path: String,
    pub sha: String,
    pub url: String,
    pub git_url: String,
    pub html_url: String,
    pub repository: Repository,
    pub score: f64,
    pub file_size: usize,
    pub lanauge: String,
    pub last_modified_at: String,
    pub line_numbers: Vec<usize>,
    pub text_matches: Vec<CodeSearchTextMatch>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CodeSearchResponse {
    pub total_count: u64,
    pub incomplete_results: bool,
    pub items: Vec<CodeSearchResult>,
}
