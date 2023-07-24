use std::sync::Arc;

use twilight_http::client::InteractionClient;
use twilight_model::{
    application::{command::CommandOptionChoice, interaction::application_command::CommandData},
    channel::message::{Component, Embed},
    id::{marker::InteractionMarker, Id},
};

use crate::types::cache::Guild;

pub struct ApplicationCommandInteraction<'a> {
    pub cached_guild: Arc<Guild>,
    pub context: ApplicationCommandInteractionContext<'a>,
    pub data: Box<CommandData>,
    pub shard_id: u64,
}

pub struct ApplicationCommandInteractionContext<'a> {
    pub id: Id<InteractionMarker>,
    pub interaction_client: InteractionClient<'a>,
    pub token: String,
}

#[derive(Default)]
pub struct AutocompletePayload {
    pub choices: Vec<CommandOptionChoice>,
}

#[derive(Default)]
pub struct DeferInteractionPayload {
    pub ephemeral: bool,
}

#[derive(Default)]
pub struct ResponsePayload {
    pub components: Vec<Component>,
    pub embeds: Vec<Embed>,
    pub ephemeral: bool,
}

#[derive(Default)]
pub struct UpdateResponsePayload {
    pub components: Vec<Component>,
    pub embeds: Vec<Embed>,
}
