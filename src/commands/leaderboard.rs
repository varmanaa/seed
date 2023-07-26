use std::time::Duration;

use futures::StreamExt;
use twilight_interactions::command::{CommandModel, CreateCommand};
use twilight_model::{
    application::interaction::{Interaction, InteractionData},
    channel::message::{
        component::{ActionRow, Button, ButtonStyle},
        Component,
        ReactionType,
    },
    id::{marker::UserMarker, Id},
};
use twilight_standby::Standby;
use twilight_util::builder::embed::EmbedBuilder;

use crate::{
    types::{
        context::Context,
        interaction::{
            ApplicationCommandInteraction,
            DeferInteractionPayload,
            UpdateResponsePayload,
        },
        Result,
    },
    utility::decimal::modulo,
};

#[derive(CommandModel, CreateCommand)]
#[command(desc = "Check Discord API latency", name = "leaderboard")]
pub struct LeaderboardCommand {}

impl LeaderboardCommand {
    pub async fn run(
        context: &Context,
        interaction: &ApplicationCommandInteraction<'_>,
    ) -> Result<()> {
        interaction
            .context
            .defer(DeferInteractionPayload {
                ephemeral: false,
            })
            .await?;

        let member_ids = interaction.cached_guild.member_ids.read().clone();
        let leaderboard = context
            .database
            .get_guild_leaderboard(interaction.cached_guild.guild_id, member_ids)
            .await?;
        let mut embed_builder = EmbedBuilder::new().color(0xF8F8FF);

        if leaderboard.is_empty() {
            embed_builder = embed_builder.description(
                "There are no members with XP
        in this guild",
            );

            interaction
                .context
                .update_response(UpdateResponsePayload {
                    embeds: vec![embed_builder.build()],
                    ..Default::default()
                })
                .await?;

            return Ok(());
        }

        let chunks = leaderboard
            .chunks(10)
            .collect::<Vec<&[(Id<UserMarker>, i64)]>>();
        let components = if chunks.len() > 1 {
            vec![Component::ActionRow(ActionRow {
                components: vec![
                    Component::Button(Button {
                        custom_id: Some("previous".to_owned()),
                        disabled: false,
                        emoji: Some(ReactionType::Unicode {
                            name: "⬅️".to_owned(),
                        }),
                        label: None,
                        style: ButtonStyle::Primary,
                        url: None,
                    }),
                    Component::Button(Button {
                        custom_id: Some("next".to_owned()),
                        disabled: false,
                        emoji: Some(ReactionType::Unicode {
                            name: "➡️".to_owned(),
                        }),
                        label: None,
                        style: ButtonStyle::Primary,
                        url: None,
                    }),
                ],
            })]
        } else {
            Vec::new()
        };
        let mut index = 0;
        let get_chunk_description = |index: usize| {
            chunks[index]
                .iter()
                .filter_map(|(user_id, xp)| {
                    context
                        .cache
                        .get_member(interaction.cached_guild.guild_id, *user_id)
                        .map(|member| {
                            if member.discriminator == 0 {
                                format!("{} - {}", member.username, xp)
                            } else {
                                format!("{}#{:04} - {}", member.username, member.discriminator, xp)
                            }
                        })
                })
                .collect::<Vec<String>>()
                .join("")
        };
        let embed = EmbedBuilder::new()
            .color(0xF8F8FF)
            .description(get_chunk_description(index))
            .build();

        interaction
            .context
            .update_response(UpdateResponsePayload {
                components: components.clone(),
                embeds: vec![embed],
                ..Default::default()
            })
            .await?;

        if chunks.len() > 1 {
            let response = interaction.context.response().await?;
            let standby = Standby::new();
            let mut component_stream = Box::pin(
                standby
                    .wait_for_component_stream(response.id, |interaction: &Interaction| {
                        if let Some(InteractionData::MessageComponent(data)) = &interaction.data {
                            vec!["previous", "next"].contains(&data.custom_id.as_str())
                        } else {
                            false
                        }
                    })
                    .take_until(tokio::time::sleep(Duration::from_secs(15))),
            );

            while let Some(i) = component_stream.next().await {
                let Some(InteractionData::MessageComponent(data)) = i.data else {
                    continue;
                };

                index = match data.custom_id.as_str() {
                    "next" => modulo(index + 1, chunks.len()),
                    _ => modulo(index - 1, chunks.len()),
                };

                let embed = EmbedBuilder::new()
                    .color(0xF8F8FF)
                    .description(get_chunk_description(index))
                    .build();

                interaction
                    .context
                    .update_response(UpdateResponsePayload {
                        components: components.clone(),
                        embeds: vec![embed],
                        ..Default::default()
                    })
                    .await?;
            }
        }

        Ok(())
    }
}
