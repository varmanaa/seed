use time::OffsetDateTime;
use tokio_postgres::types::ToSql;
use twilight_model::id::{
    marker::{GuildMarker, UserMarker},
    Id,
};

use crate::types::{database::Database, Result};

impl Database {
    pub async fn get_member(
        &self,
        guild_id: Id<GuildMarker>,
        user_id: Id<UserMarker>,
    ) -> Result<Option<(i64, Option<OffsetDateTime>)>> {
        let client = self.pool.get().await?;
        let statement = "
            SELECT
                xp,
                last_message_timestamp
            FROM
                public.member
            WHERE
                guild_id = $1
                AND user_id = $2;
        ";
        let params: &[&(dyn ToSql + Sync)] = &[&(guild_id.get() as i64), &(user_id.get() as i64)];
        let member = client
            .query_one(statement, params)
            .await
            .map_or(None, |row| {
                Some((
                    row.get::<_, i64>("xp"),
                    row.get::<_, Option<OffsetDateTime>>("last_message_timestamp"),
                ))
            });

        Ok(member)
    }

    pub async fn get_members(
        &self,
        guild_id: Id<GuildMarker>,
    ) -> Result<Vec<(Id<UserMarker>, i64, Option<OffsetDateTime>)>> {
        let client = self.pool.get().await?;
        let statement = "
            SELECT
                user_id,
                xp,
                last_message_timestamp
            FROM
                public.member
            WHERE
                guild_id = $1;
        ";
        let params: &[&(dyn ToSql + Sync)] = &[&(guild_id.get() as i64)];
        let members = client
            .query(statement, params)
            .await?
            .into_iter()
            .map(|row| {
                (
                    Id::<UserMarker>::new(row.get::<_, i64>("user_id") as u64),
                    row.get::<_, i64>("xp"),
                    row.get::<_, Option<OffsetDateTime>>("last_message_timestamp"),
                )
            })
            .collect();

        Ok(members)
    }

    pub async fn update_member_xp(
        &self,
        guild_id: Id<GuildMarker>,
        user_id: Id<UserMarker>,
        updated_xp: i64,
        last_message_timestamp: Option<OffsetDateTime>,
    ) -> Result<()> {
        let client = self.pool.get().await?;
        let statement = "
            INSERT INTO
                public.member (guild_id, user_id, xp, last_message_timestamp)
            VALUES
                ($1, $2, $3, $4)
            ON CONFLICT (guild_id, user_id)
            DO UPDATE
            SET
                xp = $3,
                last_message_timestamp = COALESCE($4, EXCLUDED.last_message_timestamp);
        ";
        let params: &[&(dyn ToSql + Sync)] = &[
            &(guild_id.get() as i64),
            &(user_id.get() as i64),
            &updated_xp,
            &last_message_timestamp,
        ];

        client.execute(statement, params).await?;

        Ok(())
    }
}
