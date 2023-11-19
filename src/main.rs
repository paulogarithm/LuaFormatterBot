use std::env;

use serenity::async_trait;  
use serenity::model::channel::Message;
use serenity::model::gateway::Ready;
use serenity::prelude::*;
use serenity::utils::MessageBuilder;

mod luaformat;
use luaformat::extract_codeblocks;

struct Handler;

#[async_trait]
impl EventHandler for Handler {
  async fn message(&self, context: Context, message: Message) {
    // To avoid recursion (the bot handling their own messages)
    if message.author.bot { return }

    // Get lines and treat to blocks
    let lines: Vec<String> = message.content.split('\n').map(String::from).collect();
    let blocks = extract_codeblocks(lines);
    if blocks.is_empty() {
      return;
    }

    // Initialize the buffer
    let mut buffer: String = MessageBuilder::new()
      .push_bold_safe(&message.author.name)
      .push(" doesn't know how to format.\n")
      .push("But don't worry, I am here !\n\n")
      .build();

    // Fill the buffer with blocks
    let mut is_a_code = false;

    for block in blocks {
      if block.1 && !is_a_code {
        buffer.push_str("```lua\n");
      } else if !block.1 && is_a_code {
        buffer.push_str("```\n");
      }

      is_a_code = block.1;
      buffer.push_str((block.0 + "\n").as_str());
    }

    if is_a_code {
      buffer.push_str("```\n");
    }

    // Send message and delete the previous message
    message.channel_id.say(&context.http, &buffer).await.expect("Could not send the formatted message");
    message.delete(&context.http).await.expect("Could not delete the old message");
  }

  async fn ready(&self, _: Context, client: Ready) {
    println!("[INFO] {} is connected!", client.user.name);
  }
}

#[tokio::main]
async fn main() {
  let token: String = env::var("TOKEN").expect("Expected a token in the environment");
  let intents: GatewayIntents = GatewayIntents::GUILD_MESSAGES
    | GatewayIntents::DIRECT_MESSAGES
    | GatewayIntents::MESSAGE_CONTENT;

  let mut client: Client = Client::builder(token, intents)
    .event_handler(Handler)
    .await.expect("Error creating client!");

  if let Err(why) = client.start().await {
    match why {
      SerenityError::Gateway(e) => eprintln!("[ERR] Gateway error: {}", e),
      _ => eprintln!("[ERR] Client error: {:?}", why),
    }
  }
}
