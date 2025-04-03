use crate::helpers::logger::Logger;
use sqlx::PgPool;

pub struct SqlxPgPool<'a, L: Logger> {
    logger: &'a L,
}

impl<'a, L: Logger> SqlxPgPool<'a, L> {
    pub fn new(logger: &'a L) -> Self {
        Self { logger }
    }

    pub async fn connect(&self, database_url: &str) -> Result<PgPool, sqlx::Error> {
        self.logger.log("Initializing postgres pool connection..");
        PgPool::connect(database_url).await
    }
}
