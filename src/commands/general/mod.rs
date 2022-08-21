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
#[commands(echo, select_random, split)]
struct General;

mod echo;
use echo::*;

mod random;
use random::*;
