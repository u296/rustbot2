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
}
