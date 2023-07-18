use std::collections::{HashMap, HashSet};

use time::OffsetDateTime;
use tokio_postgres::types::ToSql;
use twilight_model::id::{
    marker::{GuildMarker, RoleMarker, UserMarker},
    Id,
};

use crate::types::{database::Database, Result};

impl Database {
    pub async fn get_members(
        &self,
        guild_id: Id<GuildMarker>,
    ) -> Result<HashMap<Id<UserMarker>, (Option<OffsetDateTime>, HashSet<Id<RoleMarker>>)>> {
        let client = self.pool.get().await?;
        let statement = "
            SELECT
                user_id,
                last_message_timestamp,
                owned_role_ids
            FROM
                public.member
            WHERE
                guild_id = $1;
        ";
        let params: &[&(dyn ToSql + Sync)] = &[&(guild_id.get() as i64)];
        let mut members: HashMap<
            Id<UserMarker>,
            (Option<OffsetDateTime>, HashSet<Id<RoleMarker>>),
        > = HashMap::new();
        let rows = client.query(statement, params).await.unwrap_or_default();

        for row in rows {
            members.insert(
                Id::<UserMarker>::new(row.get::<_, i64>("user_id") as u64),
                (
                    row.get::<_, Option<OffsetDateTime>>("last_message_timestamp"),
                    row.get::<_, Vec<i64>>("owned_role_ids")
                        .into_iter()
                        .map(|id| Id::<RoleMarker>::new(id as u64))
                        .collect(),
                ),
            );
        }

        Ok(members)
    }

    pub async fn insert_member(
        &self,
        guild_id: Id<GuildMarker>,
        user_id: Id<UserMarker>,
    ) -> Result<(Option<OffsetDateTime>, HashSet<Id<RoleMarker>>)> {
        let client = self.pool.get().await?;
        let statement = "
            INSERT INTO
                public.member (guild_id, user_id)
            VALUES
                ($1, $2)
            ON CONFLICT
            DO NOTHING
            RETURNING
                last_message_timestamp,
                owned_role_ids;
        ";
        let params: &[&(dyn ToSql + Sync)] = &[&(guild_id.get() as i64), &(user_id.get() as i64)];
        let (last_message_timestamp, owned_role_ids) = client
            .query_one(statement, params)
            .await
            .map_or((None, HashSet::new()), |row| {
                (
                    row.get::<_, Option<OffsetDateTime>>("last_message_timestamp"),
                    row.get::<_, Vec<i64>>("owned_role_ids")
                        .into_iter()
                        .map(|id| Id::<RoleMarker>::new(id as u64))
                        .collect(),
                )
            });

        Ok((last_message_timestamp, owned_role_ids))
    }

    pub async fn remove_owned_role(
        &self,
        guild_id: Id<GuildMarker>,
        role_id: Id<RoleMarker>,
    ) -> Result<Vec<(Id<UserMarker>, HashSet<Id<RoleMarker>>)>> {
        let client = self.pool.get().await?;
        let statement = "
            UPDATE
                public.member
            SET
                owned_role_ids = ARRAY_REMOVE(owned_role_ids, $2)
            WHERE
                guild_id = $1
                AND owned_role_ids && ARRAY[$2]
            RETURNING
                user_id,
                owned_role_ids;
        ";
        let params: &[&(dyn ToSql + Sync)] = &[&(guild_id.get() as i64), &(role_id.get() as i64)];
        let updated_members = client
            .query(statement, params)
            .await?
            .into_iter()
            .map(|row| {
                (
                    Id::<UserMarker>::new(row.get::<_, i64>("user_id") as u64),
                    row.get::<_, Vec<i64>>("owned_role_ids")
                        .into_iter()
                        .map(|id| Id::<RoleMarker>::new(id as u64))
                        .collect(),
                )
            })
            .collect::<Vec<(Id<UserMarker>, HashSet<Id<RoleMarker>>)>>();

        Ok(updated_members)
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

    pub async fn update_member_owned_role_ids(
        &self,
        guild_id: Id<GuildMarker>,
        user_id: Option<Id<UserMarker>>,
        owned_role_ids: HashSet<Id<RoleMarker>>,
    ) -> Result<()> {
        let client = self.pool.get().await?;
        let statement = "
            UPDATE
                public.member
            SET
                owned_role_ids = $3
            WHERE
                guild_id = $1
                AND ($2::INT8 OR user_id = $2)
        ";
        let params: &[&(dyn ToSql + Sync)] = &[
            &(guild_id.get() as i64),
            &(user_id.map(|id| id.get() as i64)),
            &(owned_role_ids
                .into_iter()
                .map(|id| id.get() as i64)
                .collect::<Vec<i64>>()),
        ];

        client.execute(statement, params).await?;

        Ok(())
    }
}
