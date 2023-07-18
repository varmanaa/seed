use tokio_postgres::types::ToSql;
use twilight_model::id::{marker::GuildMarker, Id};

use crate::types::{database::Database, Result};

impl Database {
    pub async fn insert_guild(
        &self,
        guild_id: Id<GuildMarker>,
    ) -> Result<i64> {
        let client = self.pool.get().await?;
        let statement = "
            INSERT INTO
                public.guild (guild_id)
            VALUES
                ($1)
            ON CONFLICT
            DO NOTHING
            RETURNING
                xp_multiplier;
        ";
        let params: &[&(dyn ToSql + Sync)] = &[&(guild_id.get() as i64)];
        let xp_multiplier = client
            .query_one(statement, params)
            .await
            .map_or(1i64, |row| row.get::<_, i64>("xp_multiplier"));

        Ok(xp_multiplier)
    }

    pub async fn remove_guild(
        &self,
        guild_id: Id<GuildMarker>,
    ) -> Result<()> {
        let client = self.pool.get().await?;
        let statement = "
            DELETE FROM
                public.guild
            WHERE
                guild_id = $1;
        ";
        let params: &[&(dyn ToSql + Sync)] = &[&(guild_id.get() as i64)];

        client.execute(statement, params).await?;

        Ok(())
    }
}
