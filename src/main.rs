mod commands;
mod state;

use commands::slash::SlashCommand;

use std::env;
use std::sync::Arc;

use serenity::all::{
    CreateInteractionResponse, CreateInteractionResponseMessage, GuildId, Interaction, Ready,
};
use serenity::async_trait;
use serenity::prelude::*;

struct Handler {
    state_handle: state::StateHandle,
}

#[async_trait]
impl EventHandler for Handler {
    async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
        if let Interaction::Command(command) = interaction {
            let name = command.data.name.as_str();

            println!("Received command: {}", name);

            let content = match name {
                "ping" => Some(
                    commands::ping::Ping::new(self.state_handle.clone())
                        .run(&command.data.options()),
                ),
                "ferris-says" => Some(
                    commands::ferris_says::FerrisSays::new(self.state_handle.clone())
                        .run(&command.data.options()),
                ),
                _ => Some("not implemented :(".to_string()),
            };


            if let Some(content) = content {
                let data = CreateInteractionResponseMessage::new().content(content);
                let builder = CreateInteractionResponse::Message(data);
                if let Err(why) = command.create_response(&ctx.http, builder).await {
                    println!("Cannot respond to slash command: {why}");
                }
            }
        }
    }

    async fn ready(&self, ctx: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);

        let guild_id = GuildId::new(
            env::var("GUILD_ID")
                .expect("Expected GUILD_ID in environment")
                .parse()
                .expect("GUILD_ID must be an integer"),
        );

        let commands = guild_id
            .set_commands(&ctx.http, vec![commands::ping::Ping::register(), commands::ferris_says::FerrisSays::register()])
            .await;

        println!(
            "I setup {} slash command(s)!",
            commands.map(|c| c.len()).unwrap()
        );
    }
}

#[tokio::main]
async fn main() {
    // Login with a bot token from the environment
    let token = env::var("DISCORD_TOKEN").expect("Expected a token in the environment");
    // Set gateway intents, which decides what events the bot will be notified about
    let intents = GatewayIntents::GUILD_MESSAGES | GatewayIntents::MESSAGE_CONTENT;

    let state = state::BotState::new();

    let state_handle: state::StateHandle = Arc::new(Mutex::new(state));

    // Create a new instance of the Client, logging in as a bot.
    let mut client = Client::builder(&token, intents)
        .event_handler(Handler { state_handle })
        .await
        .expect("Err creating client");

    // Start listening for events by starting a single shard
    if let Err(why) = client.start().await {
        println!("Client error: {why:?}");
    }
}
