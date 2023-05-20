use std::ops::Deref;

use serenity::builder::CreateApplicationCommand;
use serenity::model::prelude::command::CommandOptionType;
use serenity::model::prelude::interaction::application_command::{
    ApplicationCommandInteraction, CommandDataOptionValue,
};
use serenity::model::prelude::*;
use serenity::prelude::*;

use crate::config::{Config, ConfigData};

pub async fn run(
    ctx: &mut Context,
    command: &ApplicationCommandInteraction,
    dbcon: &sqlx::SqlitePool,
) -> String {
    let calling_guild = command.guild_id.expect("failed to get guild_id");
    let calling_channel = command.channel_id;
    let calling_member = command
        .member
        .clone()
        .expect("failed to get command member.")
        .user
        .id;

    let mut data = ctx.data.write().await;
    let config = data.get_mut::<ConfigData>().unwrap();

    if crate::permissions::is_operator(
        calling_member.as_u64(),
        calling_guild.as_u64(),
        dbcon,
        config,
    )
    .await
    {
        if crate::permissions::is_serious_channel(
            &calling_channel.as_u64(),
            &calling_guild.as_u64(),
            dbcon,
        )
        .await
        {
            crate::permissions::remove_serious_channel(
                &calling_channel.as_u64(),
                &calling_guild.as_u64(),
                dbcon,
            )
            .await;
            return format!(
                "This channel has been marked as not serious by <@{}>",
                calling_member
            );
        } else {
            crate::permissions::add_serious_channel(
                &calling_channel.as_u64(),
                &calling_guild.as_u64(),
                dbcon,
            )
            .await;
            return format!(
                "This channel has been marked as serious by <@{}>",
                calling_member
            );
        }
    } else {
        return "You do not have permission to access that command.".into();
    }
}

pub fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    command
        .name("serious")
        .description("Toggles the serious flag for a channel")
}
