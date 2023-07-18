use std::{collections::HashSet, sync::Arc};

use parking_lot::RwLock;
use time::OffsetDateTime;
use twilight_model::id::{
    marker::{ChannelMarker, GuildMarker, RoleMarker, UserMarker},
    Id,
};

use crate::types::cache::{Cache, Member, MemberUpdate};

impl Cache {
    pub fn get_member(
        &self,
        guild_id: Id<GuildMarker>,
        user_id: Id<UserMarker>,
    ) -> Option<Arc<Member>> {
        self.members.try_read().map_or(None, |read_lock| {
            read_lock.get(&(guild_id, user_id)).cloned()
        })
    }

    pub fn insert_member(
        &self,
        guild_id: Id<GuildMarker>,
        user_id: Id<UserMarker>,
        joined_voice_timestamp: Option<OffsetDateTime>,
        last_message_timestamp: Option<OffsetDateTime>,
        owned_role_ids: HashSet<Id<RoleMarker>>,
        voice_channel_id: Option<Id<ChannelMarker>>,
    ) {
        self.members.write().insert(
            (guild_id, user_id),
            Arc::new(Member {
                guild_id,
                joined_voice_timestamp: RwLock::new(joined_voice_timestamp),
                last_message_timestamp: RwLock::new(last_message_timestamp),
                owned_role_ids: RwLock::new(owned_role_ids),
                user_id,
                voice_channel_id: RwLock::new(voice_channel_id),
            }),
        );

        let Some(current_guild) = self.get_guild(guild_id) else {
            return
        };

        current_guild.member_ids.write().insert(user_id);
    }

    pub fn remove_member(
        &self,
        guild_id: Id<GuildMarker>,
        user_id: Id<UserMarker>,
    ) {
        let Some(removed_member) = self.members.write().remove(&(guild_id, user_id)) else {
            return;
        };
        let Some(current_guild) = self.get_guild(removed_member.guild_id) else {
            return;
        };

        current_guild.member_ids.write().remove(&user_id);
    }

    pub fn update_member(
        &self,
        guild_id: Id<GuildMarker>,
        user_id: Id<UserMarker>,
        update: MemberUpdate,
    ) {
        let Some(current_member) = self.get_member(guild_id, user_id) else {
            return
        };
        let current_member_last_message_timestamp =
            current_member.last_message_timestamp.read().clone();
        let current_member_owned_role_ids = current_member.owned_role_ids.read().clone();
        let current_member_joined_voice_timestamp =
            current_member.joined_voice_timestamp.read().clone();
        let current_member_voice_channel_id = current_member.voice_channel_id.read().clone();

        self.members.write().insert(
            (guild_id, user_id),
            Arc::new(Member {
                guild_id,
                joined_voice_timestamp: RwLock::new(
                    update
                        .joined_voice_timestamp
                        .unwrap_or(current_member_joined_voice_timestamp),
                ),
                last_message_timestamp: RwLock::new(
                    update
                        .last_message_timestamp
                        .unwrap_or(current_member_last_message_timestamp),
                ),
                owned_role_ids: RwLock::new(
                    update
                        .owned_role_ids
                        .unwrap_or(current_member_owned_role_ids),
                ),
                user_id,
                voice_channel_id: RwLock::new(
                    update
                        .voice_channel_id
                        .unwrap_or(current_member_voice_channel_id),
                ),
            }),
        );
    }
}
