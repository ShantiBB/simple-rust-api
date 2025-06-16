use sqlx::{PgPool, migrate::MigrateDatabase, Postgres};

#[derive(Clone)]
pub struct PostgresRepository {
    pub pool: PgPool,
}

impl PostgresRepository {
    pub async fn new(db_url: &str) -> Result<Self, sqlx::Error> {
        // Создаем базу данных если она не существует
        if !Postgres::database_exists(db_url).await.unwrap_or(false) {
            log::info!("Creating database...");
            Postgres::create_database(db_url).await?;
        }

        let pool = PgPool::connect(db_url).await?;
        Ok(Self { pool })
    }

    pub async fn run_migrations(&self) -> Result<(), sqlx::migrate::MigrateError> {
        log::info!("Running database migrations...");
        sqlx::migrate!("./migrations").run(&self.pool).await?;
        log::info!("Database migrations completed successfully");
        Ok(())
    }
}
