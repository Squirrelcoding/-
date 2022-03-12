use clap::{Parser, Subcommand};
use serenity::{
    async_trait,
    model::{
        channel::{Message, ReactionType},
        gateway::Ready,
    },
    prelude::*,
};
mod lib;
use lib::*;

struct Handler;
#[async_trait]
impl EventHandler for Handler {
    async fn message(&self, ctx: Context, msg: Message) {
        if let Err(why) = msg
            .channel_id
            .create_reaction(&ctx.http, msg.id, ReactionType::Unicode("🗿".to_string()))
            .await
        {
            println!("Error sending message: {:?}", why);
        }
    }

    async fn ready(&self, _: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);
    }
}

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    #[clap(subcommand)]
    command: Option<Commands>,
}
#[derive(Subcommand, Debug)]
enum Commands {
    /// Sets a secret to use when running the bot
    SetSecret {
        ///The key of the bot to be used
        #[clap(short, long)]
        secret: String,
    },
    ///Runs the bot
    Run,
}

#[tokio::main]
async fn main() {
    let cli = Args::parse();

    match &cli.command {
        Some(Commands::SetSecret { secret }) => {
            set_secret(secret);
        }
        Some(Commands::Run) => {
            let token = get_secret();
            let mut client = Client::builder(&token)
                .event_handler(Handler)
                .await
                .expect("Err creating client");
            if let Err(why) = client.start().await {
                println!("Client error: {:?}", why);
                println!("Make sure you have set a valid key. If you haven't set one up already, run moyaibot set-secret --secret <YOUR KEY> ");
            }
        }
        _ => {
            println!("Unkown command. Run moyaibot --help for more information.")
        }
    }
}
