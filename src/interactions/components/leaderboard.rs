use std::sync::Arc;

use thousands::Separable;
use twilight_util::builder::embed::{EmbedBuilder, EmbedFooterBuilder};

use crate::{
    types::{
        cache::Member,
        context::Context,
        interaction::{MessageComponentInteraction, UpdatePayload},
        Result,
    },
    utility::decimal::modulo,
};

pub struct LeaderboardComponent {}

impl LeaderboardComponent {
    pub async fn run(
        context: &Context,
        interaction: &MessageComponentInteraction<'_>,
    ) -> Result<()> {
        let footer_text = &interaction.message.embeds[0].footer.as_ref().unwrap().text;
        let mut split = footer_text.split(" ");
        let current_index = split.nth(1).unwrap().parse::<usize>()? - 1;
        let mut leaderboard = interaction
            .cached_guild
            .member_ids
            .read()
            .iter()
            .filter_map(|user_id| {
                let member = context
                    .cache
                    .get_member(interaction.cached_guild.guild_id, *user_id);

                match member {
                    Some(member) if member.xp.read().gt(&0) => Some(member),
                    _ => None,
                }
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

        let total_pages = (leaderboard.len() as f32 / 10.0).ceil() as usize;
        let new_index = if interaction.data.custom_id.as_str().ends_with("next") {
            modulo(total_pages + current_index + 1, total_pages)
        } else {
            modulo(total_pages + current_index - 1, total_pages)
        };
        let embed = EmbedBuilder::new()
            .color(0xF8F8FF)
            .description(
                leaderboard
                    .iter()
                    .skip(new_index * 10)
                    .take(10)
                    .enumerate()
                    .map(|(index, member)| {
                        let rank = (new_index * 10) + index + 1;
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
