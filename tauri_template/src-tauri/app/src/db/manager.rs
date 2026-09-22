use sqlx::{SqlitePool, Sqlite, Transaction, migrate::Migrator};

static MIGRATOR: Migrator = sqlx::migrate!("../migrations");

#[derive(Clone, Debug)]
pub struct DbManager {
    pool: SqlitePool,
}

impl DbManager {
    pub async fn new(pool: SqlitePool) -> Result<Self, sqlx::Error> {
        MIGRATOR.run(&pool).await?;
        Ok(Self { pool })
    }

    pub fn pool(&self) -> &SqlitePool {
        &self.pool
    }

    pub async fn begin(
        &self
    ) -> Result<Transaction<'_, Sqlite>, sqlx::Error> {
        self.pool.begin().await
    }

}