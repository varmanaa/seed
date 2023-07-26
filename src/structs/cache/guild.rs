use std::{collections::HashSet, sync::Arc};

use parking_lot::RwLock;
use time::OffsetDateTime;
use twilight_model::{
    channel::Channel as TwilightChannel,
    id::{
        marker::{ChannelMarker, GuildMarker, RoleMarker, UserMarker},
        Id,
    },
};

use crate::types::cache::{Cache, Guild, GuildUpdate};

impl Cache {
    pub fn get_guild(
        &self,
        guild_id: Id<GuildMarker>,
    ) -> Option<Arc<Guild>> {
        self.guilds
            .try_read()
            .map_or(None, |read_lock| read_lock.get(&guild_id).cloned())
    }

    pub fn insert_guild(
        &self,
        channels: Vec<TwilightChannel>,
        guild_id: Id<GuildMarker>,
        levels: Vec<(u64, HashSet<Id<RoleMarker>>)>,
        members: Vec<(
            u16,
            Option<OffsetDateTime>,
            Id<UserMarker>,
            String,
            Option<Id<ChannelMarker>>,
        )>,
        name: String,
        xp_multiplier: f64,
    ) {
        let mut channel_ids: HashSet<Id<ChannelMarker>> = HashSet::new();
        let mut member_ids: HashSet<Id<UserMarker>> = HashSet::new();

        for channel in channels {
            channel_ids.insert(channel.id);

            self.insert_channel(channel);
        }

        for (discriminator, last_message_timestamp, user_id, username, voice_channel_id) in members
        {
            member_ids.insert(user_id);

            let joined_voice_timestamp = voice_channel_id.map(|_| OffsetDateTime::now_utc());

            self.insert_member(
                discriminator,
                guild_id,
                joined_voice_timestamp,
                last_message_timestamp,
                user_id,
                username,
                voice_channel_id,
            );
        }

        self.guilds.write().insert(
            guild_id,
            Arc::new(Guild {
                channel_ids: RwLock::new(channel_ids),
                guild_id,
                levels: RwLock::new(levels),
                member_ids: RwLock::new(member_ids),
                name,
                xp_multiplier: RwLock::new(xp_multiplier),
            }),
        );
        self.remove_unavailable_guild(guild_id)
    }

    pub fn remove_guild(
        &self,
        guild_id: Id<GuildMarker>,
        unavailable: bool,
    ) {
        self.guilds.write().remove(&guild_id);

        if unavailable {
            self.insert_unavailable_guild(guild_id)
        }
    }

    pub fn update_guild(
        &self,
        guild_id: Id<GuildMarker>,
        update: GuildUpdate,
    ) {
        let Some(current_guild) = self.get_guild(guild_id) else {
            return
        };
        let current_guild_channel_ids = current_guild.channel_ids.read().clone();
        let current_guild_levels = current_guild.levels.read().clone();
        let current_guild_member_ids = current_guild.member_ids.read().clone();
        let current_guild_xp_multiplier = current_guild.xp_multiplier.read().clone();

        self.guilds.write().insert(
            guild_id,
            Arc::new(Guild {
                channel_ids: RwLock::new(update.channel_ids.unwrap_or(current_guild_channel_ids)),
                guild_id,
                levels: RwLock::new(update.levels.unwrap_or(current_guild_levels)),
                member_ids: RwLock::new(update.member_ids.unwrap_or(current_guild_member_ids)),
                name: update.name.unwrap_or(current_guild.name.clone()),
                xp_multiplier: RwLock::new(
                    update.xp_multiplier.unwrap_or(current_guild_xp_multiplier),
                ),
            }),
        );
    }
}
