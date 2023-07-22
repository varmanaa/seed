use std::collections::HashMap;

use time::OffsetDateTime;
use tokio_postgres::types::ToSql;
use twilight_model::id::{
    marker::{GuildMarker, UserMarker},
    Id,
};

use crate::types::{database::Database, Result};

impl Database {
    pub async fn get_members(
        &self,
        guild_id: Id<GuildMarker>,
    ) -> Result<HashMap<Id<UserMarker>, Option<OffsetDateTime>>> {
        let client = self.pool.get().await?;
        let statement = "
            SELECT
                user_id,
                last_message_timestamp
            FROM
                public.member
            WHERE
                guild_id = $1;
        ";
        let params: &[&(dyn ToSql + Sync)] = &[&(guild_id.get() as i64)];
        let mut members: HashMap<Id<UserMarker>, Option<OffsetDateTime>> = HashMap::new();
        let rows = client.query(statement, params).await.unwrap_or_default();

        for row in rows {
            members.insert(
                Id::<UserMarker>::new(row.get::<_, i64>("user_id") as u64),
                row.get::<_, Option<OffsetDateTime>>("last_message_timestamp"),
            );
        }

        Ok(members)
    }

    pub async fn insert_member(
        &self,
        guild_id: Id<GuildMarker>,
        user_id: Id<UserMarker>,
    ) -> Result<Option<OffsetDateTime>> {
        let client = self.pool.get().await?;
        let statement = "
            INSERT INTO
                public.member (guild_id, user_id)
            VALUES
                ($1, $2)
            ON CONFLICT
            DO NOTHING
            RETURNING
                last_message_timestamp;
        ";
        let params: &[&(dyn ToSql + Sync)] = &[&(guild_id.get() as i64), &(user_id.get() as i64)];
        let last_message_timestamp = client
            .query_one(statement, params)
            .await
            .map_or(None, |row| {
                row.get::<_, Option<OffsetDateTime>>("last_message_timestamp")
            });

        Ok(last_message_timestamp)
    }

    pub async fn update_member_xp(
        &self,
        guild_id: Id<GuildMarker>,
        user_id: Id<UserMarker>,
        amount: i64,
        last_message_timestamp: Option<OffsetDateTime>,
    ) -> Result<()> {
        let client = self.pool.get().await?;
        let statement = "
            UPDATE
                public.member
            SET
                xp = xp + $3,
                last_message_timestamp = COALESCE($4, last_message_timestamp)
            WHERE
                guild_id = $1
                AND user_id = $2;
        ";
        let params: &[&(dyn ToSql + Sync)] =
            &[&(guild_id.get() as i64), &(user_id.get() as i64), &amount, &last_message_timestamp];

        client.execute(statement, params).await?;

        Ok(())
    }
}
