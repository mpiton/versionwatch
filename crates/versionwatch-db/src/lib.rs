use versionwatch_core::domain::software_version::SoftwareVersion;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Database query failed")]
    Query(#[from] sqlx::Error),
}

pub struct Db {
    pool: sqlx::PgPool,
}

impl Db {
    pub async fn connect(database_url: &str) -> Result<Self, Error> {
        let pool = sqlx::PgPool::connect(database_url).await?;
        Ok(Self { pool })
    }

    /// Inserts or updates a software version in the database.
    pub async fn upsert_version(&self, version: &SoftwareVersion) -> Result<(), Error> {
        sqlx::query!(
            r#"
            INSERT INTO software_versions (name, latest_version, latest_lts_version, is_lts, eol_date, release_notes_url, cve_count)
            VALUES ($1, $2, $3, $4, $5, $6, $7)
            ON CONFLICT (name) DO UPDATE
            SET 
                latest_version = EXCLUDED.latest_version,
                latest_lts_version = EXCLUDED.latest_lts_version,
                is_lts = EXCLUDED.is_lts,
                eol_date = EXCLUDED.eol_date,
                release_notes_url = EXCLUDED.release_notes_url,
                cve_count = EXCLUDED.cve_count,
                updated_at = NOW()
            "#,
            version.name,
            version.latest_version,
            version.latest_lts_version,
            version.is_lts,
            version.eol_date,
            version.release_notes_url,
            version.cve_count
        )
        .execute(&self.pool)
        .await?;

        Ok(())
    }
}
