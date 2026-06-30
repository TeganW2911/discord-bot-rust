
use serenity::all::{ChannelId, UserId};
use nonempty::{NonEmpty, nonempty};
use serenity::all::{CacheHttp, Cache, Http};

use std::str::FromStr;

use crate::commands::command::*;
use crate::utility::*;
use crate::databases::*;


pub struct ContextCommand;

impl Command for ContextCommand {

    fn permission<'a>(&'a self, message: &'a MessageManager) -> BoxedFuture<'a, bool> {
        Box::pin(async move {
            message.is_trial().await
        })
    }

    fn define_usage(&self) -> UsageBuilder {
        UsageBuilder::new(
            CommandType::Moderation,
            nonempty!["context".to_string()]
        )
    }

    fn run(&self, params: CommandParams) -> BoxedFuture<'_, ()> {
        Box::pin(
            async move {

            }
        )
    }

}


