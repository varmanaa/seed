use std::collections::HashSet;

use skia_safe::{Data, Image};
use twilight_interactions::command::{CommandModel, CreateCommand};
use twilight_model::id::{marker::UserMarker, Id};

use crate::{
    types::{
        context::Context,
        interaction::{ApplicationCommandInteraction, DeferInteractionPayload, UpdatePayload},
        Result,
    },
    utility::image::get_profile,
};

#[derive(CommandModel, CreateCommand)]
#[command(desc = "View a member's rank", name = "rank")]
pub struct RankCommand {
    #[command(desc = "The member to check", rename = "member")]
    user_id: Id<UserMarker>,
}

impl RankCommand {
    pub async fn run(
        context: &Context,
        interaction: &mut ApplicationCommandInteraction<'_>,
    ) -> Result<()> {
        interaction
            .context
            .defer(DeferInteractionPayload {
                ephemeral: false,
            })
            .await?;

        let guild_id = interaction.cached_guild.guild_id;
        let Self {
            user_id,
        } = RankCommand::from_interaction(interaction.input_data())?;
        let (avatar_url, username) = if let Some(member) =
            context.cache.get_member(guild_id, user_id)
        {
            (member.avatar_url.read().clone(), member.username.clone())
        } else {
            let member = context
                .http
                .guild_member(guild_id, user_id)
                .await?
                .model()
                .await?;
            let avatar_url = if let Some(member_avatar) = member.avatar {
                format!("https://cdn.discordapp.com/guilds/{guild_id}/users/{user_id}/avatars/{member_avatar}.png")
            } else if let Some(user_avatar) = member.user.avatar {
                format!("https://cdn.discordapp.com/avatars/{user_id}/{user_avatar}.png")
            } else {
                let index = if member.user.discriminator == 0 {
                    (user_id.get() >> 22) % 6
                } else {
                    (member.user.discriminator % 5) as u64
                };

                format!("https://cdn.discordapp.com/embed/avatars/{index}.png")
            };
            let last_message_timestamp = context.database.insert_member(guild_id, user_id).await?;

            context.cache.insert_member(
                avatar_url.clone(),
                member.user.bot,
                member.user.discriminator,
                guild_id,
                None,
                last_message_timestamp,
                HashSet::from_iter(member.roles.clone()),
                user_id,
                member.user.name.clone(),
                None,
            );

            (avatar_url, member.user.name)
        };
        let member_ids = interaction.cached_guild.member_ids.read().clone();
        let (rank, _, xp) = context
            .database
            .get_guild_members(interaction.cached_guild.guild_id, member_ids, Some(user_id))
            .await?
            .first()
            .cloned()
            .unwrap();
        let formatted_uri = format!("{avatar_url}?size=512");
        let response = context.hyper.get(formatted_uri.parse()?).await?;
        let avatar_image_bytes = hyper::body::to_bytes(response.into_body()).await?;
        let avatar_image_data = Data::new_copy(&avatar_image_bytes);
        let avatar_image = Image::from_encoded(avatar_image_data).unwrap();
        let attachment = get_profile(guild_id, avatar_image, username, rank, xp);

        interaction
            .context
            .update_response(UpdatePayload {
                attachments: vec![attachment],
                ..Default::default()
            })
            .await?;

        Ok(())
    }
}
