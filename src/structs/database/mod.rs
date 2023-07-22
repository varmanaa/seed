mod guild;
mod member;

use std::str::FromStr;

use deadpool_postgres::{Manager, ManagerConfig, Pool, RecyclingMethod};
use tokio_postgres::{Config, NoTls};

use crate::{
    types::{database::Database, Result},
    utility::constants::DATABASE_URL,
};

impl Database {
    pub async fn create_tables(&self) -> Result<()> {
        let client = self.pool.get().await?;
        let statement = "
            -- guild table
            CREATE TABLE IF NOT EXISTS public.guild (
                guild_id INT8 NOT NULL PRIMARY KEY,
                xp_multiplier INT8 NOT NULL DEFAULT 1
            );

            -- member table
            CREATE TABLE IF NOT EXISTS public.member (
                guild_id INT8 NOT NULL,
                user_id INT8 NOT NULL,
                xp INT8 NOT NULL DEFAULT 0,
                last_message_timestamp TIMESTAMP WITH TIME ZONE,
                PRIMARY KEY (guild_id, user_id)
            )
        ";

        client.batch_execute(statement).await?;

        Ok(())
    }

    pub fn new() -> Result<Self> {
        Ok(Self {
            pool: Pool::builder(Manager::from_config(
                Config::from_str(DATABASE_URL.as_str())?,
                NoTls,
                ManagerConfig {
                    recycling_method: RecyclingMethod::Fast,
                },
            ))
            .max_size(16)
            .build()?,
        })
    }
}
