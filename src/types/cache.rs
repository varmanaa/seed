use std::{
    collections::{HashMap, HashSet},
    sync::Arc,
};

use parking_lot::RwLock;
use time::OffsetDateTime;
use twilight_model::id::{
    marker::{ChannelMarker, GuildMarker, RoleMarker, UserMarker},
    Id,
};

pub struct Cache {
    pub channels: RwLock<HashMap<Id<ChannelMarker>, Arc<Channel>>>,
    pub guilds: RwLock<HashMap<Id<GuildMarker>, Arc<Guild>>>,
    pub members: RwLock<HashMap<(Id<GuildMarker>, Id<UserMarker>), Arc<Member>>>,
    pub unavailable_guilds: RwLock<HashSet<Id<GuildMarker>>>,
}

pub struct Channel {
    pub channel_id: Id<ChannelMarker>,
    pub guild_id: Id<GuildMarker>,
    pub user_ids: RwLock<HashSet<Id<UserMarker>>>,
}

#[derive(Default)]
pub struct ChannelUpdate {
    pub user_ids: Option<HashSet<Id<UserMarker>>>,
}

pub struct Guild {
    pub channel_ids: RwLock<HashSet<Id<ChannelMarker>>>,
    pub guild_id: Id<GuildMarker>,
    pub levels: RwLock<Vec<(u64, HashSet<Id<RoleMarker>>)>>,
    pub member_ids: RwLock<HashSet<Id<UserMarker>>>,
    pub name: String,
    pub xp_multiplier: RwLock<f64>,
}

#[derive(Default)]
pub struct GuildUpdate {
    pub channel_ids: Option<HashSet<Id<ChannelMarker>>>,
    pub levels: Option<Vec<(u64, HashSet<Id<RoleMarker>>)>>,
    pub member_ids: Option<HashSet<Id<UserMarker>>>,
    pub name: Option<String>,
    pub xp_multiplier: Option<f64>,
}

pub struct Member {
    pub avatar_url: RwLock<String>,
    pub bot: bool,
    pub discriminator: u16,
    pub guild_id: Id<GuildMarker>,
    pub joined_voice_timestamp: RwLock<Option<OffsetDateTime>>,
    pub last_message_timestamp: RwLock<Option<OffsetDateTime>>,
    pub role_ids: RwLock<HashSet<Id<RoleMarker>>>,
    pub user_id: Id<UserMarker>,
    pub username: String,
    pub voice_channel_id: RwLock<Option<Id<ChannelMarker>>>,
    pub xp: RwLock<i64>
}

#[derive(Default)]
pub struct MemberUpdate {
    pub avatar_url: Option<String>,
    pub discriminator: Option<u16>,
    pub joined_voice_timestamp: Option<Option<OffsetDateTime>>,
    pub last_message_timestamp: Option<Option<OffsetDateTime>>,
    pub role_ids: Option<HashSet<Id<RoleMarker>>>,
    pub username: Option<String>,
    pub voice_channel_id: Option<Option<Id<ChannelMarker>>>,
    pub xp: Option<i64>
}
