use std::{collections::HashSet, sync::Arc};

use skia_safe::{Data, Image};
use twilight_interactions::command::{CommandModel, CreateCommand};
use twilight_model::id::{marker::UserMarker, Id};

use crate::{
    types::{
        cache::Member,
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
        let (avatar_url, username, xp) = if let Some(member) =
            context.cache.get_member(guild_id, user_id)
        {
            (
                member.avatar_url.read().to_owned(),
                member.username.to_owned(),
                member.xp.read().to_owned(),
            )
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
            let (xp, last_message_timestamp) = context
                .database
                .get_member(guild_id, user_id)
                .await?
                .unwrap_or_default();

            context.cache.insert_member(
                avatar_url.clone(),
                member.user.bot,
                member.user.discriminator,
                guild_id,
                None,
                last_message_timestamp,
                HashSet::from_iter(member.roles.to_owned()),
                user_id,
                member.user.name.to_owned(),
                None,
                xp,
            );

            (avatar_url, member.user.name, xp)
        };
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

        let index = leaderboard
            .into_iter()
            .position(|member| member.user_id.eq(&user_id))
            .unwrap_or(interaction.cached_guild.member_ids.read().len() - 1);
        let formatted_uri = format!("{avatar_url}?size=512");
        let response = context.hyper.get(formatted_uri.parse()?).await?;
        let avatar_image_bytes = hyper::body::to_bytes(response.into_body()).await?;
        let avatar_image_data = Data::new_copy(&avatar_image_bytes);
        let avatar_image = Image::from_encoded(avatar_image_data).unwrap();
        let attachment = get_profile(guild_id, avatar_image, username, (index + 1) as u64, xp);

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
