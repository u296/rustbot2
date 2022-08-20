mod prelude {
    pub use tracing::*;
    pub use std::error::Error;
    pub use serenity::prelude::*;
}

use std::sync::Arc;

use args::Args;
use prelude::*;
use songbird::SerenityInit;
use tokio::try_join;
use tracing_subscriber::layer::SubscriberExt;
use clap::Parser;
use serenity::{framework::StandardFramework, client::bridge::gateway::ShardManager};


mod token;
mod args;
mod eventhandler;

struct ShardManagerContainer;

impl TypeMapKey for ShardManagerContainer {
    type Value = Arc<Mutex<ShardManager>>;
}


fn main() -> Result<(), Box<dyn Error>> {
    let executor = tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()?;

    let res = executor.block_on(telemetry_setup());

    opentelemetry::global::shutdown_tracer_provider();

    res
    
}


async fn telemetry_setup() -> Result<(), Box<dyn Error>> {
    opentelemetry::global::set_text_map_propagator(opentelemetry_jaeger::Propagator::new());

    let args = args::Args::parse();


    let tracer = opentelemetry_jaeger::new_pipeline()
        .with_service_name("rustbot2")
        //.with_agent_endpoint(&args.endpoint)
        //.with_auto_split_batch(true)
        .install_simple()?;//batch(opentelemetry::runtime::Tokio)?;
    let telemetry = tracing_opentelemetry::layer().with_tracer(tracer);
    let subscriber = tracing_subscriber::Registry::default().with(telemetry);

    tracing::subscriber::set_global_default(subscriber)?;


    match run_bot(args).await {
        Err(e) => {
            error!(e);
            Err(e)
        }
        x => x
    }
    
}

#[instrument]
fn log_version_info() {
    info!(git_hash = env!("GIT_HASH"), "build commit");
}

#[instrument]
async fn run_bot(args: Args) -> Result<(), Box<dyn Error>> {
    
    log_version_info();

    let (token,) = try_join!(token::get_token(&args))?;

    info!(token);


    let framework = StandardFramework::new()
        .configure(|c|{
            c.prefix(".")
            .delimiters(vec![" "])
        });

    let gateway_intents = GatewayIntents::default()
        | GatewayIntents::GUILDS
        | GatewayIntents::GUILD_MEMBERS
        | GatewayIntents::GUILD_MESSAGES
        | GatewayIntents::GUILD_VOICE_STATES;
    
    let mut client = Client::builder(token, gateway_intents)
        .framework(framework)
        .register_songbird()
        .event_handler(eventhandler::Handler::new())
        .await?;

    {
        let mut data = client.data.write().await;
        data.insert::<ShardManagerContainer>(client.shard_manager.clone());
    }

    let shard_manager = client.shard_manager.clone();

    tokio::spawn(async move {
        tokio::signal::ctrl_c().await.expect("failed to wait for ctrl-c");
        shard_manager.lock().await.shutdown_all().await;
    });
    

    client.start().await.map_err(|e|e.into())
}

