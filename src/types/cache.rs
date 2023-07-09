use std::{
    collections::{HashMap, HashSet},
    sync::Arc,
};

use parking_lot::RwLock;
use time::OffsetDateTime;
use twilight_model::{
    guild::Permissions,
    id::{
        marker::{ChannelMarker, GuildMarker, RoleMarker, UserMarker},
        Id,
    },
    util::ImageHash,
};

pub struct Cache {
    pub guilds: RwLock<HashMap<Id<GuildMarker>, Arc<Guild>>>,
    pub members: RwLock<HashMap<(Id<GuildMarker>, Id<UserMarker>), Arc<Member>>>,
    pub roles: RwLock<HashMap<Id<RoleMarker>, Arc<Role>>>,
    pub unavailable_guilds: RwLock<HashSet<Id<GuildMarker>>>,
    pub users: RwLock<HashMap<Id<UserMarker>, Arc<User>>>,
    pub voice_states: RwLock<HashMap<(Id<GuildMarker>, Id<UserMarker>), Arc<Id<ChannelMarker>>>>,
}

pub struct Guild {
    pub channel_ids: RwLock<HashSet<Id<ChannelMarker>>>,
    pub guild_id: Id<GuildMarker>,
    pub member_ids: RwLock<HashSet<Id<UserMarker>>>,
    pub name: String,
    pub role_ids: RwLock<HashSet<Id<RoleMarker>>>,
}

#[derive(Default)]
pub struct GuildUpdate {
    pub name: Option<String>,
}

pub struct Member {
    pub avatar: Option<ImageHash>,
    pub communication_disabled_until: Option<OffsetDateTime>,
    pub deaf: bool,
    pub guild_id: Id<GuildMarker>,
    pub mute: bool,
    pub role_ids: RwLock<HashSet<Id<RoleMarker>>>,
    pub user_id: Id<UserMarker>,
}

#[derive(Default)]
pub struct MemberUpdate {
    pub avatar: Option<Option<ImageHash>>,
    pub communication_disabled_until: Option<Option<OffsetDateTime>>,
    pub deaf: Option<bool>,
    pub mute: Option<bool>,
    pub role_ids: Option<Vec<Id<RoleMarker>>>,
}

pub struct Role {
    pub guild_id: Id<GuildMarker>,
    pub permissions: Permissions,
    pub role_id: Id<RoleMarker>,
}

#[derive(Default)]
pub struct RoleUpdate {
    pub permissions: Option<Permissions>,
}

pub struct User {
    pub avatar: Option<ImageHash>,
    pub banner: Option<ImageHash>,
    pub discriminator: u16,
    pub mutual_guilds: u8,
    pub name: String,
    pub user_id: Id<UserMarker>,
}

#[derive(Default)]
pub struct UserUpdate {
    pub avatar: Option<Option<ImageHash>>,
    pub banner: Option<Option<ImageHash>>,
    pub discriminator: Option<u16>,
    pub mutual_guilds: Option<u8>,
    pub name: Option<String>,
}
