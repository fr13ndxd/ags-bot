use dotenv::dotenv;
use std::env;

use serenity::all::{ChannelId, Member};
use serenity::async_trait;
use serenity::model::channel::Message;
use serenity::model::gateway::Ready;
use serenity::prelude::*;

struct Handler;

async fn generate_ai_response(ctx: Context, msg: Message) {
    let output = std::process::Command::new("curl")
        .arg("http://localhost:11434/api/generate")
        .arg("-d")
        .arg(format!(
            r#"{{"model": "gemma:2b", "prompt": "{}"}}"#,
            msg.content.replace("\n", "\\n")
        ))
        .output()
        .expect("Failed to execute command");

    let output_str = String::from_utf8_lossy(&output.stdout);
    let lines: Vec<_> = output_str.lines().collect();

    let mut sentence = String::new();

    for line in lines {
        let json: serde_json::Value = serde_json::from_str(line).unwrap();
        let response = json["response"].as_str().unwrap();
        sentence.push_str(response);
        sentence = sentence
            .replace("@everyone", "`<everyone ping>`")
            .replace("<@&1143759057778393200>", "`<member role ping>`")
            .replace("@member", "`<member role ping>`")
            .replace("@here", "`<here ping>`");

        if let Some(done) = json.get("done") {
            if done.as_bool().unwrap() == true {
                break;
            }
        }
    }

    sentence.pop();

    msg.reply(ctx.clone(), sentence).await.unwrap();
}

#[async_trait]
impl EventHandler for Handler {
    async fn message(&self, ctx: Context, msg: Message) {
        if msg.content.starts_with("!ai") || msg.content.contains("<@1143408254127448104>") {
            let _ = msg
                .content
                .strip_prefix("!ai")
                .unwrap_or_default()
                .replace("\n", "\\n")
                .replace("<@1143408254127448104>", "");
            generate_ai_response(ctx, msg).await;
        }
    }

    async fn guild_member_addition(&self, ctx: Context, new_member: Member) {
        let channel_id = ChannelId::new(1143610931708965019);
        let welcome_message = format!("Welcome {}", new_member.mention());

        channel_id.say(&ctx, welcome_message).await.unwrap();
    }

    async fn ready(&self, _: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);
    }
}

#[tokio::main]
async fn main() {
    dotenv().ok();

    let token = dotenv::var("DISCORD_TOKEN").unwrap();

    let intents = GatewayIntents::GUILD_MESSAGES
        | GatewayIntents::DIRECT_MESSAGES
        | GatewayIntents::MESSAGE_CONTENT
        | GatewayIntents::GUILD_MEMBERS;

    let mut client = Client::builder(&token, intents)
        .event_handler(Handler)
        .await
        .expect("Err creating client");

    if let Err(why) = client.start().await {
        println!("Client error: {why:?}");
    }
}
