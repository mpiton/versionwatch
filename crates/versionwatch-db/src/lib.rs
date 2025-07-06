use versionwatch_core::domain::product_cycle::ProductCycle;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Database query failed")]
    Query(#[from] sqlx::Error),
    #[error("Migration failed")]
    Migrate(#[from] sqlx::migrate::MigrateError),
}

pub struct Db {
    pool: sqlx::PgPool,
}

impl Db {
    pub async fn connect(database_url: &str) -> Result<Self, Error> {
        let pool = sqlx::PgPool::connect(database_url).await?;
        sqlx::migrate!().run(&pool).await?;
        Ok(Self { pool })
    }

    pub async fn upsert_product(&self, name: &str) -> Result<i32, Error> {
        let rec = sqlx::query!(
            r#"
            INSERT INTO products (name, display_name)
            VALUES ($1, $1)
            ON CONFLICT (name) DO UPDATE
            SET name = EXCLUDED.name
            RETURNING id
            "#,
            name
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(rec.id)
    }

    pub async fn upsert_cycle(&self, product_id: i32, cycle: &ProductCycle) -> Result<(), Error> {
        sqlx::query!(
            r#"
            INSERT INTO cycles (product_id, name, release_date, eol_date, lts)
            VALUES ($1, $2, $3, $4, $5)
            ON CONFLICT (product_id, name) DO UPDATE
            SET
                release_date = EXCLUDED.release_date,
                eol_date = EXCLUDED.eol_date,
                lts = EXCLUDED.lts,
                updated_at = NOW()
            "#,
            product_id,
            cycle.name,
            cycle.release_date,
            cycle.eol_date,
            cycle.lts
        )
        .execute(&self.pool)
        .await?;

        Ok(())
    }
}
