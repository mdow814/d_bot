mod utils;

use std::env;
use serenity::async_trait;
use serenity::client::{Client, Context, EventHandler};
use serenity::model::channel::Message;
use serenity::framework::standard::{
    StandardFramework,
    CommandResult,
    macros::{
        command,
        group
    }
};


#[group]
#[commands(ping, build, game)]
struct General;

struct Handler;

#[async_trait]
impl EventHandler for Handler {}

#[tokio::main]
async fn main() {
    let framework = StandardFramework::new()
        .configure(|c| c.prefix("~"))
        .group(&GENERAL_GROUP);

    let token = env::var("DISCORD_TOKEN").expect("token");
    let mut client = Client::builder(token)
        .event_handler(Handler)
        .framework(framework)
        .await
        .expect("Error creating client");

    match client.start().await {
        Ok(_)    => println!("Started client successfully"),
        Err(why) => println!("Could not start the client: {:?}", why)
    }
}

#[command]
async fn ping(ctx: &Context, msg: &Message) -> CommandResult {
    msg.reply(ctx, "Pong!").await?;

    Ok(())
}

#[command]
async fn build(ctx: &Context, msg: &Message) -> CommandResult {
    let mut build = utils::builds::get_build();
    let message = build.get_items();
    msg.reply(ctx, message).await?;

    Ok(())
}

#[command]
async fn game(ctx: &Context, msg: &Message) -> CommandResult {
    let game = utils::games::get_game();
    msg.reply(ctx, game).await?;
    Ok(())
}