use thousands::Separable;
use twilight_util::builder::embed::{EmbedBuilder, EmbedFooterBuilder};

use crate::{
    types::{
        context::Context,
        interaction::{MessageComponentInteraction, UpdatePayload},
        Result,
    },
    utility::decimal::modulo,
};

pub struct PreviousComponent {}

impl PreviousComponent {
    pub async fn run(
        context: &Context,
        interaction: &MessageComponentInteraction<'_>,
    ) -> Result<()> {
        let footer_text = &interaction.message.embeds[0].footer.as_ref().unwrap().text;
        let mut split = footer_text.split(" ");
        let current_index = split.nth(1).unwrap().parse::<usize>()? - 1;
        let member_ids = interaction.cached_guild.member_ids.read().clone();
        let leaderboard = context
            .database
            .get_guild_members(interaction.cached_guild.guild_id, member_ids, None)
            .await?;
        let total_pages = (leaderboard.len() as f32 / 10.0).ceil() as usize;
        let new_index = modulo(current_index - 1, total_pages);
        let embed = EmbedBuilder::new()
            .color(0xF8F8FF)
            .description(
                leaderboard
                    .iter()
                    .skip(new_index * 10)
                    .take(10)
                    .filter_map(|(rank, user_id, xp)| {
                        context
                            .cache
                            .get_member(interaction.cached_guild.guild_id, *user_id)
                            .map(|member| {
                                let username = if member.discriminator == 0 {
                                    member.username.clone()
                                } else {
                                    format!("{}#{:04}", member.username, member.discriminator)
                                };

                                format!(
                                    "#{} - **{}** ({} XP)",
                                    rank,
                                    username,
                                    xp.separate_with_commas()
                                )
                            })
                    })
                    .collect::<Vec<String>>()
                    .join("\n"),
            )
            .footer(EmbedFooterBuilder::new(format!(
                "Page {} of {total_pages}",
                new_index + 1
            )))
            .title(format!("{} leaderboard", interaction.cached_guild.name))
            .build();

        interaction
            .context
            .update_message(UpdatePayload {
                components: interaction.message.components.clone(),
                embeds: vec![embed.clone()],
                ..Default::default()
            })
            .await?;

        Ok(())
    }
}
