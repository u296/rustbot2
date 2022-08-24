use async_trait::async_trait;

use super::prelude::*;

#[derive(Debug)]
pub struct Handler {}

impl Handler {
    pub fn new() -> Self {
        Self {}
    }
}

#[async_trait]
impl EventHandler for Handler {
    #[instrument(skip(_ctx))]
    async fn ready(&self, _ctx: Context, _info: serenity::model::gateway::Ready) {
        info!("READY");
        info!(
            git_hash = env!("GIT_HASH"),
            "git hash: {}",
            env!("GIT_HASH")
        );
        println!("READY");
        println!("git hash: {}", env!("GIT_HASH"));
    }

    #[instrument(skip(ctx))]
    async fn voice_state_update(
        &self,
        ctx: Context,
        _old: Option<serenity::model::voice::VoiceState>,
        new: serenity::model::voice::VoiceState,
    ) {
        match new.guild_id {
            Some(guild_id) => {
                let manager = songbird::get(&ctx).await.unwrap();

                match manager.get(guild_id) {
                    Some(call) => {
                        info!("guild has handler");

                        let lock = call.lock().await;

                        match lock.current_channel() {
                            Some(connected_channel_id) => {
                                info!("handler is connected");
                                let voice_states = ctx
                                    .cache
                                    .guild_field(guild_id, |guild| guild.voice_states.clone())
                                    .unwrap();

                                for (user_id, voice_state) in voice_states.iter() {
                                    if voice_state.channel_id.map(Into::into)
                                        == Some(connected_channel_id)
                                    {
                                        let user = match user_id.to_user(&ctx).await {
                                            Ok(u) => u,
                                            Err(e) => {
                                                error!("error getting user: {}", e);
                                                return;
                                            }
                                        };

                                        info!(user_name = user.name, user_id = user.id.0);

                                        if !user.bot {
                                            info!("there is a real user in the call");
                                            return;
                                        }
                                    }
                                }
                            }
                            None => (),
                        }

                        info!("no real people in call, removing handler");

                        drop(lock);

                        match manager.remove(guild_id).await {
                            Ok(()) => (),
                            Err(e) => {
                                error!("failed to remove handler: {}", e);
                            }
                        }

                        info!("successfully removed handler");
                    }
                    None => {
                        info!("guild has no handler");
                    }
                }
            }
            None => {
                warn!("voice state update didn't contain guild id!");
            }
        }
    }
}
