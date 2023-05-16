#![allow(dead_code)]
#![allow(unused_imports)]

mod commands;
mod config;

use std::env;

use serenity::async_trait;
use serenity::model::application::command::Command;
use serenity::model::application::interaction::{Interaction, InteractionResponseType};
use serenity::model::channel::Message;
use serenity::model::gateway::Ready;
use serenity::model::id::{ChannelId, CommandId, GuildId, MessageId};
use serenity::prelude::*;

use config::Config;

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
        if let Interaction::ApplicationCommand(command) = interaction {
            println!("Received command interaction: {:#?}", command);

            let content = match command.data.name.as_str() {
                "ping" => commands::ping::run(&command.data.options),
                "op" => commands::op::run(&command),
                //"attachmentinput" => commands::attachmentinput::run(&command.data.options),
                _ => "not implemented :(".to_string(),
            };

            if let Err(why) = command
                .create_interaction_response(&ctx.http, |response| {
                    response
                        .kind(InteractionResponseType::ChannelMessageWithSource)
                        .interaction_response_data(|message| message.content(content))
                })
                .await
            {
                println!("Cannot respond to slash command: {}", why);
            }
        }
    }

    async fn ready(&self, ctx: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);

        let guild_id = GuildId(
            env::var("GUILD_ID")
                .expect("Did not find guild_id in config")
                .parse()
                .expect("guild_id must be an integer"),
        );

        let commands = GuildId::set_application_commands(&guild_id, &ctx.http, |commands| {
            commands
                .create_application_command(|command| commands::ping::register(command))
                .create_application_command(|command| commands::op::register(command))

            /*.create_application_command(|command| commands::id::register(command))
            .create_application_command(|command| commands::welcome::register(command))
            .create_application_command(|command| commands::numberinput::register(command))
            .create_application_command(|command| commands::attachmentinput::register(command))*/
        })
        .await
        .expect("Failed to register commands.");

        /*Command::delete_global_application_command(&ctx.http, CommandId::from(1107873714357403718))
        .await
        .expect("Errored");*/
        println!(
            "I now have the following guild slash commands: {:?}",
            commands
        );

        /*let guild_command = Command::create_global_application_command(&ctx.http, |command| {
            commands::ping::register(command)
        })
        .await;

        println!(
            "I created the following global slash command: {:#?}",
            guild_command
        );*/
    }
}

#[tokio::main]
async fn main() {
    // configure the bot.
    let config = Config::read_from_file("config.toml");
    env::set_var("GUILD_ID", config.discord.guild_id);
    //print oauth join link
    println!(
        "Use this link to add the bot to your server: https://discord.com/api/oauth2/authorize?client_id={}&permissions={}&scope={}",
        config.discord.client_id, config.discord.perm_int, config.discord.scope
    );

    // Set gateway intents, which decides what events the bot will be notified about
    let intents = GatewayIntents::GUILD_MESSAGES
        | GatewayIntents::DIRECT_MESSAGES
        | GatewayIntents::MESSAGE_CONTENT;

    // Create a new instance of the Client, logging in as a bot. This will
    // automatically prepend your bot token with "Bot ", which is a requirement
    // by Discord for bot users.
    let mut client = Client::builder(&config.discord.token, intents)
        .event_handler(Handler)
        .await
        .expect("Error creating client");

    // Finally, start a single shard, and start listening to events.
    //
    // Shards will automatically attempt to reconnect, and will perform
    // exponential backoff until it reconnects.
    if let Err(why) = client.start().await {
        println!("Client error: {:?}", why);
    }
}
