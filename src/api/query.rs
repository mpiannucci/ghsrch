/// How to deal with forks in search results
pub enum ForkSearchBehavior {
    /// Do not include forks in search results
    NoForks,
    /// Include forks in search results
    IncludeForks,
    /// Only search repositories that are forks
    OnlyForks,
    /// Only search repositories with more than the given number of forks
    ForkCount(u32),
}

impl From<&ForkSearchBehavior> for String {
    fn from(behavior: &ForkSearchBehavior) -> Self {
        match behavior {
            ForkSearchBehavior::NoForks => "".into(),
            ForkSearchBehavior::IncludeForks => "+fork:true".into(),
            ForkSearchBehavior::OnlyForks => "+fork:only".into(),
            ForkSearchBehavior::ForkCount(count) => format!("+forks:>{count}"),
        }
    }
}

/// Constructs a github search query string from the given parameters
pub struct GithubSearchQuery {
    /// The search term
    term: String,
    /// The language to limit search to
    language: Option<String>,
    /// The username to limit search to
    username: Option<String>,
    /// The filename to limit search to
    filename: Option<String>,
    /// The organization to limit search to
    organization: Option<String>,
    /// The repository to limit search to
    repository: Option<String>,
    /// How to deal with forks
    forks: ForkSearchBehavior,
}

impl GithubSearchQuery {
    /// Create a new GithubSearchQuery with the given search term
    pub fn new(term: String) -> Self {
        Self {
            term,
            username: None,
            filename: None,
            language: None,
            organization: None,
            repository: None,
            forks: ForkSearchBehavior::NoForks,
        }
    }

    /// Add a username to the search query to only search this user
    pub fn with_user(&mut self, username: String) -> &mut Self {
        self.username = Some(username);
        self
    }

    /// Add a filename to the search query to only search files with the given filename
    pub fn with_filename(&mut self, filename: String) -> &mut Self {
        self.filename = Some(filename);
        self
    }

    /// Add a language to the search query to only search files with the given language
    pub fn with_language(&mut self, language: String) -> &mut Self {
        self.language = Some(language);
        self
    }

    /// Add an organization to the search query to only search files in the given organization
    pub fn with_organization(&mut self, organization: String) -> &mut Self {
        self.organization = Some(organization);
        self
    }

    /// Add a repository to the search query to only search files in the given repository
    pub fn with_repository(&mut self, repository: String) -> &mut Self {
        self.repository = Some(repository);
        self
    }

    /// Set the fork search behavior
    pub fn with_forks(&mut self, forks: ForkSearchBehavior) -> &mut Self {
        self.forks = forks;
        self
    }

    /// Build the query string for the search request
    pub fn build(&self) -> String {
        let username = if let Some(username) = self.username.as_ref() {
            format!("+user:{}", username)
        } else {
            "".into()
        };

        let filename = if let Some(filename) = self.filename.as_ref() {
            format!("+in:{}", filename)
        } else {
            "".into()
        };

        let language = if let Some(language) = self.language.as_ref() {
            format!("+language:{}", language)
        } else {
            "".into()
        };

        let organization = if let Some(organization) = self.organization.as_ref() {
            format!("+org:{}", organization)
        } else {
            "".into()
        };

        let repository = if let Some(repository) = self.repository.as_ref() {
            format!("+repo:{}", repository)
        } else {
            "".into()
        };

        let forks: String = String::from(&self.forks);

        format!(
            "{term}{username}{filename}{language}{organization}{repository}{forks}",
            term = self.term
        )
    }
}

#[cfg(test)]
mod tests {
    use super::GithubSearchQuery;

    #[test]
    fn build_search_query() {
        let simple = GithubSearchQuery::new("test".into()).build();
        assert_eq!(simple, "test");

        let complex = GithubSearchQuery::new("test".into())
            .with_user("testuser".into())
            .with_filename("readme".into())
            .build();
        assert_eq!(complex, "test+user:testuser+in:readme");

        let more_complex = GithubSearchQuery::new("test".into())
            .with_user("testuser".into())
            .with_filename("readme".into())
            .with_language("rust".into())
            .with_organization("testorg".into())
            .with_repository("testuser/testrepo".into())
            .build();
        assert_eq!(
            more_complex,
            "test+user:testuser+in:readme+language:rust+org:testorg+repo:testuser/testrepo"
        );
    }
}
