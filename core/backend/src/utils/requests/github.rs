use std::time::{Duration, Instant};
use std::sync::{Arc, Mutex, OnceLock};

use octocrab::models::Repository;
use octocrab::{Error as GithubError, Octocrab};
use serde::{Deserialize, Serialize};
use thiserror::Error;

const CACHE_EXPIRATION: u64 = 3600 * 3;
static REPOSITORY_CACHE: Mutex<OnceLock<RepositoryCache>> = Mutex::new(OnceLock::new());

#[derive(Error, Debug)]
pub enum GithubRequestError {
    #[error("The cache was poisoned by someone else.")]
    PoisonedCache,

    #[error("Couldn't make a request to github: {0:#}")]
    GithubRequest(#[from] GithubError)
}

struct RepositoryCache {
    expiration: Instant,
    repos: Arc<Vec<Repository>>
}

#[derive(Serialize, Deserialize)]
pub struct AppRepository {
    name: String,
    description: Option<String>,
    url: String,
    star_count: String,
    language: String,
    fork_count: String
}

impl RepositoryCache {
    #[inline]
    fn new(repos: Vec<Repository>) -> Self {
        Self {
            repos: Arc::new(repos),
            expiration: Instant::now() + Duration::from_secs(CACHE_EXPIRATION)
        }
    }

    fn has_expired(&self) -> bool {
        self.expiration.elapsed() > Duration::from_secs(CACHE_EXPIRATION)
    }

    #[inline]
    fn expiration(&self) -> Instant {
        self.expiration
    }

    #[inline]
    fn repos(&self) -> Arc<Vec<Repository>> {
        self.repos
            .clone()
    }

    #[inline]
    fn set_repos(&mut self, repos: Vec<Repository>) {
        self.repos = Arc::new(repos);
        // reset expiration after setting repos.
        self.expiration = Instant::now() + Duration::from_secs(CACHE_EXPIRATION);
    }
}

pub async fn fetch_repositories(
    instance: &Octocrab,
    username: &str
) -> Result<Arc<Vec<Repository>>, GithubRequestError> {
    async fn fetch(
        instance: &Octocrab,
        username: &str
    ) -> Result<Vec<Repository>, GithubRequestError> {
        let mut new_repos = Vec::new();
        let mut page: u32 = 0;

        loop {
            let repos = instance.users(username)
                .repos()
                .per_page(100)
                .page(page)
                .send()
                .await?;

            page += 1;
            let mut items = repos.items;

            new_repos.append(&mut items);

            if items.len() != 100 {
                break;
            }
        }

        Ok(new_repos)
    }

    let mut cache = REPOSITORY_CACHE
        .lock()
        .map_err(|_| GithubRequestError::PoisonedCache)?;

    {
        if let Some(cache) = cache.get_mut() {
            if !cache.has_expired() {
                return Ok(
                    cache.repos()
                        .clone()
                );
            }

            cache.set_repos(fetch(instance, username).await?);
        }
    }

    let repos = fetch(instance, username).await?;

    Ok(
        cache.get_or_init(|| RepositoryCache::new(repos))
            .repos()
            .clone()
    )
}
