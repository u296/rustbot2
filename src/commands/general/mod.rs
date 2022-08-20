mod prelude {
    pub use serenity::{
        framework::standard::{macros::command, Args, CommandResult},
        model::channel::Message,
        prelude::*,
    };
}

use serenity::framework::standard::macros::group;

#[group]
#[commands(ping)]
struct General;

mod ping;
use ping::*;
