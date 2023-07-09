use twilight_http::client::InteractionClient;
use twilight_model::{
    application::{command::CommandOptionChoice, interaction::application_command::CommandData},
    channel::message::{Component, Embed},
    id::{
        marker::{GuildMarker, InteractionMarker},
        Id,
    },
};

pub struct ApplicationCommandInteraction<'a> {
    pub context: ApplicationCommandInteractionContext<'a>,
    pub data: Box<CommandData>,
    pub guild_id: Id<GuildMarker>,
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
