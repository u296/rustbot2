mod prelude {
    pub use serenity::{
        framework::standard::{macros::command, Args, CommandResult},
        model::channel::Message,
        prelude::*,
    };
    pub use super::super::prelude::*;
}

use serenity::framework::standard::macros::group;

#[group]
#[commands(ping)]
struct General;

mod ping;
use ping::*;
