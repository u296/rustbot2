use async_trait::async_trait;
use serenity::client::bridge::gateway::ShardMessenger;

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
        println!("READY");
    }
}
