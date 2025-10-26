use std::{ops::Deref, sync::Arc};

use sqlx::PgPool;

pub struct GenericRepository {
    pool: Arc<PgPool>
}

impl GenericRepository {
    #[inline]
    pub const fn new(pool: Arc<PgPool>) -> Self {
        Self {
            pool
        }
    }

    #[inline]
    pub fn pool(&self) -> &PgPool {
        &self.pool
    }
}

pub trait RepositoryModel: Deref<Target = GenericRepository> + Sized {
    fn repository(pool: Arc<PgPool>) -> Self;
}
