#![allow(dead_code)]
#![allow(unused_imports)]

mod commands;
mod config;
mod database;
mod permissions;

use std::env;

use serenity::async_trait;
use serenity::model::application::command::Command;
use serenity::model::application::interaction::{Interaction, InteractionResponseType};
use serenity::model::channel::Message;
use serenity::model::gateway::Ready;
use serenity::model::id::{ChannelId, CommandId, GuildId, MessageId};
use serenity::prelude::*;

use crate::config::{Config, ConfigData};

struct Handler {
    database: sqlx::SqlitePool,
}

#[async_trait]
impl EventHandler for Handler {
    async fn interaction_create(&self, mut ctx: Context, interaction: Interaction) {
        if let Interaction::ApplicationCommand(command) = interaction {
            println!("Received command interaction: {:?}", command);

            let content: String = match command.data.name.as_str() {
                "amisus" => commands::amisus::run(&mut ctx, &command, &self.database).await,
                "ping" => commands::ping::run(&command),
                "op" => commands::op::run(&mut ctx, &command, &self.database).await,
                "deop" => commands::deop::run(&mut ctx, &command, &self.database).await,
                "balun_ban" => commands::ban::run(&mut ctx, &command, &self.database).await,
                "balun_unban" => commands::unban::run(&mut ctx, &command, &self.database).await,
                "serious" => commands::serious::run(&mut ctx, &command, &self.database).await,
                "help" => commands::help::run(&command.data.options),
                "truncate" => commands::truncate::run(&mut ctx, &command, &self.database).await,
                //"attachmentinput" => commands::attachmentinput::run(&command.data.options),
                _ => "not implemented :(".to_string(),
            };

            if content.contains("NOT_ALLOWED_huaeouiyt") {
                println!("Cannot respond to slash command: NOT_ALLOWED triggered.");
            } else {
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
    }

    async fn ready(&self, ctx: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);
        let data = ctx.data.read().await;
        let config = data.get::<ConfigData>().unwrap();
        for guild in config.discord.guild_ids.clone() {
            let guild_id = GuildId(guild as u64);

            let commands = GuildId::set_application_commands(&guild_id, &ctx.http, |commands| {
            commands
                .create_application_command(|command| commands::amisus::register(command))
                .create_application_command(|command| commands::ping::register(command))
                .create_application_command(|command| commands::deop::register(command))
                .create_application_command(|command| commands::op::register(command))
                .create_application_command(|command| commands::ban::register(command))
                .create_application_command(|command| commands::unban::register(command))
                .create_application_command(|command| commands::serious::register(command))
                .create_application_command(|command| commands::truncate::register(command))

            /*.create_application_command(|command| commands::id::register(command))
            .create_application_command(|command| commands::welcome::register(command))
            .create_application_command(|command| commands::numberinput::register(command))
            .create_application_command(|command| commands::attachmentinput::register(command))*/
        })
        .await
        .expect(format!("Failed to register commands for guild id: {}", guild).as_str());

            /*Command::delete_global_application_command(&ctx.http, CommandId::from(1107873714357403718))
            .await
            .expect("Errored");*/
            println!(
                "I now have the following slash commands for guild {}: {:?}",
                guild, commands
            );
        }

        let global_command = Command::create_global_application_command(&ctx.http, |command| {
            commands::help::register(command)
        })
        .await;

        println!(
            "I created the following global slash command: {:?}",
            global_command
        );
    }
}

#[tokio::main]
async fn main() {
    // configure the bot.
    let config = Config::read_from_file("config.toml");
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
        .event_handler(Handler {
            database: database::setup(&config).await,
        })
        .await
        .expect("Error creating client");

    //make sure we drop the reference to `client` before continuing.
    {
        //add the config to the client
        let mut data = client.data.write().await;
        data.insert::<ConfigData>(config);
    }

    // Finally, start a single shard, and start listening to events.
    //
    // Shards will automatically attempt to reconnect, and will perform
    // exponential backoff until it reconnects.
    client.start().await.expect("Failed to initialize client");
}
