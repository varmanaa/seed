mod events;
mod interactions;
mod structs;
mod types;
mod utility;

use std::{collections::HashMap, sync::Arc};

use dotenv::dotenv;
use futures::StreamExt;
use twilight_gateway::{error::ReceiveMessageErrorType, stream::ShardEventStream};
use twilight_http::Client;
use twilight_model::gateway::CloseCode;

use crate::{
    types::{cache::Cache, context::Context, database::Database},
    utility::{
        constants::BOT_TOKEN,
        gateway::{connect, reconnect},
    },
};

#[tokio::main]
async fn main() -> types::Result<()> {
    dotenv().ok();

    let http = Client::new(BOT_TOKEN.to_owned());
    let application = http.current_user_application().await?.model().await?;
    let cache = Cache::new();
    let database = Database::new()?;
    let mut shards = connect(&http, HashMap::default()).await?;
    let context = Arc::new(Context::new(application, cache, database, http));

    context.database.create_tables().await?;

    let commands = interactions::commands::get_commands();

    context
        .http
        .interaction(context.application_id)
        .set_global_commands(&commands)
        .await?;

    'outer: loop {
        let mut stream = ShardEventStream::new(shards.iter_mut());

        'inner: loop {
            let error = match stream.next().await {
                None => return Ok(()),
                Some((_, Err(error))) => error,
                Some((shard, Ok(event))) => {
                    let shard_id = shard.id().number();
                    let shard_sender = shard.sender();

                    context
                        .latencies
                        .write()
                        .insert(shard_id, Arc::new(shard.latency().clone()));

                    let event_context = Arc::clone(&context);

                    tokio::spawn(async move {
                        events::handle_event(event_context, shard_id, shard_sender, event)
                            .await
                            .unwrap()
                    });

                    continue 'inner;
                }
            };
            let should_reconnect = matches!(
                error.kind(),
                ReceiveMessageErrorType::FatallyClosed {
                    close_code: CloseCode::ShardingRequired | CloseCode::UnknownError
                }
            );

            if should_reconnect {
                drop(stream);

                reconnect(&context.http, &mut shards).await?;

                continue 'outer;
            }
            if error.is_fatal() {
                return Ok(());
            }
        }
    }
}
