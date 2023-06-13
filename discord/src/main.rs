use std::env;

use serenity::async_trait;
use serenity::framework::standard::macros::{command, group};
use serenity::framework::standard::{CommandResult, StandardFramework};
use serenity::model::channel::Message;
use serenity::model::Timestamp;
use serenity::prelude::*;

#[group]
#[commands(ping, message)]
struct General;

struct Handler;

#[async_trait]
impl EventHandler for Handler {}

#[tokio::main]
async fn main() {
    let framework = StandardFramework::new()
        .configure(|c| c.prefix("~")) // set the bot's prefix to "~"
        .group(&GENERAL_GROUP);

    // Login with a bot token from the environment
    let token = env::var("DISCORD_TOKEN").expect("token");
    let intents = GatewayIntents::non_privileged() | GatewayIntents::MESSAGE_CONTENT;
    let mut client = Client::builder(token, intents)
        .event_handler(Handler)
        .framework(framework)
        .await
        .expect("Error creating client");

    // start listening for events by starting a single shard
    if let Err(why) = client.start().await {
        println!("An error occurred while running the client: {:?}", why);
    }
}

#[command]
async fn ping(ctx: &Context, msg: &Message) -> CommandResult {
    msg.reply(ctx, "Pong!").await?;

    Ok(())
}

#[command]
async fn message(ctx: &Context, msg: &Message) -> CommandResult {
    // The create message builder allows you to easily create embeds and messages
    // using a builder syntax.
    // This example will create a message that says "Hello, World!", with an embed that has
    // a title, description, an image, three fields, and a footer.

    msg.channel_id
        .send_message(&ctx.http, |m| {
            m.content("Hello, World!")
                .embed(|e| {
                    e.title("This is a title")
                        .description("This is a description")
                        .image("attachment://team_schedule.png")
                        .fields(vec![
                            ("This is the first field", "This is a field body", true),
                            ("This is the second field", "Both fields are inline", true),
                        ])
                        .field(
                            "This is the third field",
                            "This is not an inline field",
                            false,
                        )
                        .footer(|f| f.text("This is a footer"))
                        // Add a timestamp for the current time
                        // This also accepts a rfc3339 Timestamp
                        .timestamp(Timestamp::now())
                })
                .add_file("./team_schedule.png")
        })
        .await
        .expect("error sending message");

    Ok(())
}
