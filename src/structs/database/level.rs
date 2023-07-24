use std::collections::HashSet;

use tokio_postgres::types::ToSql;
use twilight_model::id::{
    marker::{GuildMarker, RoleMarker},
    Id,
};

use crate::types::{database::Database, Result};

impl Database {
    pub async fn get_levels(
        &self,
        guild_id: Id<GuildMarker>,
    ) -> Result<Vec<(u64, HashSet<Id<RoleMarker>>)>> {
        let client = self.pool.get().await?;
        let statement = "
            SELECT
                level,
                role_ids
            FROM
                public.level
            WHERE
                guild_id = $1;
        ";
        let params: &[&(dyn ToSql + Sync)] = &[&(guild_id.get() as i64)];
        let level_roles = client
            .query(statement, params)
            .await?
            .into_iter()
            .map(|row| {
                (
                    row.get::<_, i64>("level") as u64,
                    row.get::<_, Vec<i64>>("role_ids")
                        .into_iter()
                        .map(|id| Id::new(id as u64))
                        .collect::<HashSet<Id<RoleMarker>>>(),
                )
            })
            .collect::<Vec<(u64, HashSet<Id<RoleMarker>>)>>();

        Ok(level_roles)
    }

    pub async fn insert_level(
        &self,
        guild_id: Id<GuildMarker>,
        level: u64,
        role_ids: HashSet<Id<RoleMarker>>,
    ) -> Result<()> {
        let client = self.pool.get().await?;
        let statement = "
            INSERT INTO
                public.level
            VALUES
                ($1, $2, $3)
            ON CONFLICT (guild_id, level)
            DO UPDATE
            SET
                role_ids = $3;
        ";
        let params: &[&(dyn ToSql + Sync)] = &[
            &(guild_id.get() as i64),
            &(level as i64),
            &role_ids
                .into_iter()
                .map(|role_id| role_id.get() as i64)
                .collect::<Vec<i64>>(),
        ];

        client.execute(statement, params).await?;

        Ok(())
    }

    pub async fn remove_guild_levels(
        &self,
        guild_id: Id<GuildMarker>,
    ) -> Result<()> {
        let client = self.pool.get().await?;
        let statement = "
            DELETE FROM
                public.level
            WHERE
                guild_id = $1;
        ";
        let params: &[&(dyn ToSql + Sync)] = &[&(guild_id.get() as i64)];

        client.execute(statement, params).await?;

        Ok(())
    }

    pub async fn remove_level(
        &self,
        guild_id: Id<GuildMarker>,
        level: u64,
    ) -> Result<()> {
        let client = self.pool.get().await?;
        let statement = "
            DELETE FROM
                public.level
            WHERE
                guild_id = $1
                AND level = $2;
        ";
        let params: &[&(dyn ToSql + Sync)] = &[&(guild_id.get() as i64), &(level as i16)];

        client.execute(statement, params).await?;

        Ok(())
    }

    pub async fn update_guild_levels(
        &self,
        guild_id: Id<GuildMarker>,
        role_ids: HashSet<Id<RoleMarker>>,
    ) -> Result<()> {
        let client = self.pool.get().await?;
        let statement = "
            UPDATE
                public.level
            SET
                role_ids = ARRAY(
                    SELECT
                        *
                    FROM
                        UNNEST(role_ids)
                    WHERE
                        UNNEST = ANY($2)
                )
            WHERE
                guild_id = $1;
        ";
        let params: &[&(dyn ToSql + Sync)] = &[
            &(guild_id.get() as i64),
            &(role_ids
                .into_iter()
                .map(|role_id| role_id.get() as i64)
                .collect::<Vec<i64>>()),
        ];

        client.execute(statement, params).await?;

        Ok(())
    }
}
