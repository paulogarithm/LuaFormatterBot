use serenity::async_trait;
use serenity::model::channel::Message;
use serenity::model::gateway::Ready;
use serenity::prelude::*;
use serenity::utils::MessageBuilder;

mod luaformat;
use luaformat::get_blocks;

struct Handler;

async fn ping(context: Context, msg: Message) {
    msg.reply(&context, "Pong!")
        .await
        .expect("Error sending message");

    let message: String = MessageBuilder::new().push("Hello").build();

    msg.channel_id
        .say(&context.http, &message)
        .await
        .expect("Error sending message");
    return;
}

#[async_trait]
impl EventHandler for Handler {
    async fn message(&self, context: Context, msg: Message) {
        // To avoid recursion
        if msg.author.bot {
            return;
        }

        // Just for the ping command
        if msg.content == "::ping" {
            ping(context, msg).await;
            return;
        }

        // Get lines and treat to blocks
        let lines: Vec<String> = msg.content.split('\n').map(String::from).collect();
        let blocks = get_blocks(lines);
        if blocks.is_empty() {
            return;
        }

        // Initialize the buffer
        let mut buffer: String = MessageBuilder::new()
            .push_bold_safe(&msg.author.name)
            .push(" dont know how to format.\n")
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
        msg.channel_id
            .say(&context.http, &buffer)
            .await
            .expect("Error sending message");
        msg.delete(&context.http)
            .await
            .expect("Cant delete message");
    }

    async fn ready(&self, _: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);
    }
}

#[tokio::main]
async fn main() {
    // let token: String = env::var("TOKEN").expect("Expected a token in the environment");
    let token: String =
        String::from("<token>"); // Env vars doesnt work

    let intents: GatewayIntents = GatewayIntents::GUILD_MESSAGES
        | GatewayIntents::DIRECT_MESSAGES
        | GatewayIntents::MESSAGE_CONTENT;

    let mut client: Client = Client::builder(&token, intents)
        .event_handler(Handler)
        .await
        .expect("Err creating client");

    if let Err(why) = client.start().await {
        println!("Client error: {:?}", why);
    }
}
