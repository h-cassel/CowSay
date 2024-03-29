mod commands;
mod klipper;
mod socket;
mod state;

use socket::KlippyConnection;
use tokio::join;

use std::env;
use std::sync::Arc;

use serenity::all::{
    CreateInteractionResponse, CreateInteractionResponseMessage, GuildId, Interaction, Ready,
};
use serenity::prelude::*;
use serenity::{async_trait, FutureExt};

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
                        .run(&command.data.options())
                        .await,
                ),
                "info" => Some(
                    commands::info::Info::new(self.state_handle.clone())
                        .run(&command.data.options())
                        .await,
                ),
                "brightness" => Some(
                    commands::brightness::Brightness::new(self.state_handle.clone())
                        .run(&command.data.options())
                        .await,
                ),
                "ferris-says" => Some(
                    commands::ferris_says::FerrisSays::new(self.state_handle.clone())
                        .run(&command.data.options())
                        .await,
                ),
                "cancel" => Some(
                    commands::cancel::Cancel::new(self.state_handle.clone())
                        .run(&command.data.options())
                        .await,
                ),
                "pause" => Some(
                    commands::pause::Pause::new(self.state_handle.clone())
                        .run(&command.data.options())
                        .await,
                ),
                "resume" => Some(
                    commands::resume::Resume::new(self.state_handle.clone())
                        .run(&command.data.options())
                        .await,
                ),
                "estop" => Some(
                    commands::estop::Estop::new(self.state_handle.clone())
                        .run(&command.data.options())
                        .await,
                ),
                "print" => Some(
                    commands::queue::add::Add::new(self.state_handle.clone())
                        .run(&command.user, &command.data.options())
                        .await,
                ),
                "queue" => Some(
                    commands::queue::view::View::new(self.state_handle.clone())
                        .run(&command.data.options())
                        .await,
                ),
                "clear-queue" => Some(
                    commands::queue::clear::Clear::new(self.state_handle.clone())
                        .run(&command.data.options())
                        .await,
                ),
                "progress" => Some(
                    commands::progress::Progress::new(self.state_handle.clone())
                        .run(&command.data.options())
                        .await,
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
            .set_commands(
                &ctx.http,
                vec![
                    commands::ping::Ping::register(),
                    commands::info::Info::register(),
                    commands::brightness::Brightness::register(),
                    commands::ferris_says::FerrisSays::register(),
                    commands::cancel::Cancel::register(),
                    commands::pause::Pause::register(),
                    commands::resume::Resume::register(),
                    commands::estop::Estop::register(),
                    commands::queue::add::Add::register(),
                    commands::queue::view::View::register(),
                    commands::queue::clear::Clear::register(),
                    commands::progress::Progress::register(),
                ],
            )
            .await;

        println!(
            "I setup {} slash command(s)!",
            commands.map(|c| c.len()).unwrap()
        );
    }
}

const DEFAULT_SOCK_PATH: &str = "/home/pi/printer_data/comms/klippy.sock";

#[tokio::main]
async fn main() {
    println!("Loading environment variables...");
    dotenv::dotenv().ok();
    let token = env::var("DISCORD_TOKEN").expect("Expected a token in the environment");
    let intents = GatewayIntents::GUILD_MESSAGES | GatewayIntents::MESSAGE_CONTENT;

    let socket_path = env::var("KLIP_SOCK_PATH").unwrap_or(DEFAULT_SOCK_PATH.to_string());
    println!("Connecting to Klippy at {}", socket_path);
    let mut conn = KlippyConnection::new(socket_path).await;

    let state = state::BotState::new();

    let state_handle: state::StateHandle = Arc::new(Mutex::new(state));

    let tx = state_handle.lock().await.resp_channel.0.clone();
    let rx = state_handle.lock().await.req_channel.0.subscribe();

    println!("Starting bot...");
    let mut client = Client::builder(&token, intents)
        .event_handler(Handler { state_handle })
        .await
        .expect("Err creating client");

    let a = { client.start().map(|r| r.ok()) };

    println!("Starting main event loop");

    join!(a, conn.req_resp_loop(tx, rx));
}
