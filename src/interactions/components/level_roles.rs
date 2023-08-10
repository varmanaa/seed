use twilight_util::builder::embed::{EmbedBuilder, EmbedFieldBuilder, EmbedFooterBuilder};

use crate::{
    types::{
        context::Context,
        interaction::{MessageComponentInteraction, UpdatePayload},
        Result,
    },
    utility::decimal::modulo,
};

pub struct LevelRolesComponent {}

impl LevelRolesComponent {
    pub async fn run(
        _context: &Context,
        interaction: &MessageComponentInteraction<'_>,
    ) -> Result<()> {
        let footer_text = &interaction.message.embeds[0].footer.as_ref().unwrap().text;
        let mut split = footer_text.split(" ");
        let current_index = split.nth(1).unwrap().parse::<usize>()? - 1;

        let mut guild_levels = interaction.cached_guild.levels.read().clone();

        guild_levels.sort_unstable_by_key(|guild_level| guild_level.0);

        let total_pages = (guild_levels.len() as f32 / 5.0).ceil() as usize;
        let new_index = if interaction.data.custom_id.as_str().ends_with("next") {
            modulo(total_pages + current_index + 1, total_pages)
        } else {
            modulo(total_pages + current_index - 1, total_pages)
        };
        let mut embed_builder = EmbedBuilder::new()
            .color(0xF8F8FF)
            .footer(EmbedFooterBuilder::new(format!(
                "Page {} of {total_pages}",
                new_index + 1
            )))
            .title(format!("{} level role(s)", interaction.cached_guild.name));

        for (level, role_ids) in guild_levels.iter().skip(new_index * 5).take(5) {
            embed_builder = embed_builder.field(
                EmbedFieldBuilder::new(
                    format!("Level {level}"),
                    role_ids
                        .into_iter()
                        .map(|role_id| format!("- <@&{role_id}>"))
                        .collect::<Vec<String>>()
                        .join("\n"),
                )
                .build(),
            )
        }

        let embed = embed_builder.build();

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
