mod prelude {
    pub use super::super::prelude::*;
    pub use serenity::{
        framework::standard::{macros::command, Args, CommandResult},
        model::channel::Message,
        prelude::*,
    };
}



use serenity::framework::standard::macros::group;

#[group]
#[commands(play)]
struct Voice;

mod play;
use play::*;