use std::{sync::Arc, time::Duration};

use thousands::Separable;
use tokio::time::sleep;
use twilight_interactions::command::{CommandModel, CreateCommand};
use twilight_model::channel::message::{
    component::{ActionRow, Button, ButtonStyle},
    Component,
    ReactionType,
};
use twilight_util::builder::embed::{EmbedBuilder, EmbedFooterBuilder};

use crate::types::{
    cache::Member,
    context::Context,
    interaction::{ApplicationCommandInteraction, DeferInteractionPayload, UpdatePayload},
    Result,
};

#[derive(CommandModel, CreateCommand)]
#[command(desc = "View the server's leaderboard", name = "leaderboard")]
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

        let mut leaderboard = interaction
            .cached_guild
            .member_ids
            .read()
            .iter()
            .filter_map(|user_id| {
                context
                    .cache
                    .get_member(interaction.cached_guild.guild_id, *user_id)
            })
            .collect::<Vec<Arc<Member>>>();

        leaderboard.sort_unstable_by(|a, b| {
            if !b.xp.read().eq(&*a.xp.read()) {
                b.xp.read().cmp(&a.xp.read())
            } else if !b
                .last_message_timestamp
                .read()
                .eq(&a.last_message_timestamp.read())
            {
                b.last_message_timestamp
                    .read()
                    .cmp(&a.last_message_timestamp.read())
            } else {
                b.joined_voice_timestamp
                    .read()
                    .cmp(&a.joined_voice_timestamp.read())
            }
        });

        let mut embed_builder = EmbedBuilder::new()
            .color(0xF8F8FF)
            .title(format!("{} leaderboard", interaction.cached_guild.name));

        if leaderboard.is_empty() {
            embed_builder = embed_builder.description("There are no members with XP in this guild");

            interaction
                .context
                .update_response(UpdatePayload {
                    embeds: vec![embed_builder.build()],
                    ..Default::default()
                })
                .await?;

            return Ok(());
        }

        embed_builder = embed_builder.description(
            leaderboard
                .iter()
                .take(10)
                .enumerate()
                .map(|(index, member)| {
                    let rank = index + 1;
                    let username = if member.discriminator == 0 {
                        member.username.clone()
                    } else {
                        format!("{}#{:04}", member.username, member.discriminator)
                    };

                    format!(
                        "#{} - **{}** ({} XP)",
                        rank,
                        username,
                        member.xp.read().separate_with_commas()
                    )
                })
                .collect::<Vec<String>>()
                .join("\n"),
        );

        if leaderboard.len() > 10 {
            embed_builder = embed_builder.footer(EmbedFooterBuilder::new(format!(
                "Page 1 of {}",
                (leaderboard.len() as f32 / 10.0).ceil()
            )))
        }

        let components = if leaderboard.len() > 10 {
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
        let embed = embed_builder.build();

        interaction
            .context
            .update_response(UpdatePayload {
                components: components.clone(),
                embeds: vec![embed.clone()],
                ..Default::default()
            })
            .await?;

        sleep(Duration::from_secs(15)).await;

        interaction
            .context
            .update_response(UpdatePayload {
                embeds: interaction.context.response().await?.embeds,
                ..Default::default()
            })
            .await?;

        Ok(())
    }
}
