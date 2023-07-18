use core::matches;
use std::{collections::HashSet, sync::Arc};

use parking_lot::RwLock;
use twilight_model::{
    channel::{Channel as TwilightChannel, ChannelType},
    id::{marker::ChannelMarker, Id},
};

use crate::types::cache::{Cache, Channel, ChannelUpdate};

impl Cache {
    pub fn get_channel(
        &self,
        channel_id: Id<ChannelMarker>,
    ) -> Option<Arc<Channel>> {
        self.channels
            .try_read()
            .map_or(None, |read_lock| read_lock.get(&channel_id).cloned())
    }

    pub fn insert_channel(
        &self,
        channel: TwilightChannel,
    ) {
        if !matches!(
            channel.kind,
            ChannelType::GuildStageVoice | ChannelType::GuildVoice
        ) {
            return;
        }

        let Some(guild_id) = channel.guild_id else {
            return
        };

        self.channels.write().insert(
            channel.id,
            Arc::new(Channel {
                channel_id: channel.id,
                guild_id,
                user_ids: RwLock::new(HashSet::new()),
            }),
        );

        let Some(guild) = self.get_guild(guild_id) else {
            return
        };

        guild.channel_ids.write().insert(channel.id);
    }

    pub fn remove_channel(
        &self,
        channel_id: Id<ChannelMarker>,
    ) {
        let Some(removed_channel) = self.channels.write().remove(&channel_id) else {
            return
        };
        let Some(current_guild) = self.get_guild(removed_channel.guild_id) else {
            return;
        };

        current_guild.channel_ids.write().remove(&channel_id);
    }

    pub fn update_channel(
        &self,
        channel_id: Id<ChannelMarker>,
        update: ChannelUpdate,
    ) {
        let Some(current_channel) = self.get_channel(channel_id) else {
            return
        };
        let current_channel_user_ids = current_channel.user_ids.read().clone();

        self.channels.write().insert(
            channel_id,
            Arc::new(Channel {
                channel_id: current_channel.channel_id,
                guild_id: current_channel.guild_id,
                user_ids: RwLock::new(update.user_ids.unwrap_or(current_channel_user_ids)),
            }),
        );
    }
}
