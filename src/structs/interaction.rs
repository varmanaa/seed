use std::{borrow::Cow, mem::take};

use twilight_interactions::command::CommandInputData;
use twilight_model::{
    channel::{message::MessageFlags, Message},
    http::interaction::{InteractionResponse, InteractionResponseData, InteractionResponseType},
};

use crate::types::{
    interaction::{
        ApplicationCommandInteraction,
        DeferInteractionPayload,
        InteractionContext,
        ResponsePayload,
        UpdatePayload,
    },
    Result,
};

impl ApplicationCommandInteraction<'_> {
    pub fn input_data(&mut self) -> CommandInputData {
        CommandInputData {
            options: take(&mut self.data.options),
            resolved: self.data.resolved.take().map(Cow::Owned),
        }
    }
}

impl InteractionContext<'_> {
    pub async fn defer(
        &self,
        payload: DeferInteractionPayload,
    ) -> Result<()> {
        let response = InteractionResponse {
            data: Some(InteractionResponseData {
                flags: payload.ephemeral.then(|| MessageFlags::EPHEMERAL),
                ..Default::default()
            }),
            kind: InteractionResponseType::DeferredChannelMessageWithSource,
        };

        self.interaction_client
            .create_response(self.id, &self.token, &response)
            .await?;

        Ok(())
    }

    pub async fn respond(
        &self,
        payload: ResponsePayload,
    ) -> Result<()> {
        let components = if payload.components.is_empty() {
            None
        } else {
            Some(payload.components)
        };
        let embeds = if payload.embeds.is_empty() {
            None
        } else {
            Some(payload.embeds)
        };
        let response = InteractionResponse {
            data: Some(InteractionResponseData {
                components,
                embeds,
                flags: payload.ephemeral.then(|| MessageFlags::EPHEMERAL),
                ..Default::default()
            }),
            kind: InteractionResponseType::ChannelMessageWithSource,
        };

        self.interaction_client
            .create_response(self.id, &self.token, &response)
            .await?;

        Ok(())
    }

    pub async fn response(&self) -> Result<Message> {
        let message = self
            .interaction_client
            .response(&self.token)
            .await?
            .model()
            .await?;

        Ok(message)
    }

    pub async fn update_message(
        &self,
        payload: UpdatePayload,
    ) -> Result<()> {
        let components = if payload.components.is_empty() {
            None
        } else {
            Some(payload.components)
        };
        let embeds = if payload.embeds.is_empty() {
            None
        } else {
            Some(payload.embeds)
        };
        let response = InteractionResponse {
            data: Some(InteractionResponseData {
                components,
                embeds,
                ..Default::default()
            }),
            kind: InteractionResponseType::UpdateMessage,
        };

        self.interaction_client
            .create_response(self.id, &self.token, &response)
            .await?;

        Ok(())
    }

    pub async fn update_response(
        &self,
        payload: UpdatePayload,
    ) -> Result<()> {
        let components = if payload.components.is_empty() {
            None
        } else {
            Some(payload.components.as_slice())
        };
        let embeds = if payload.embeds.is_empty() {
            None
        } else {
            Some(payload.embeds.as_slice())
        };

        self.interaction_client
            .update_response(&self.token)
            .attachments(payload.attachments.as_slice())?
            .components(components)?
            .embeds(embeds)?
            .await?;

        Ok(())
    }
}
