mod capitalizer;
mod commands;

use serenity::{
    async_trait,
    model::{
        application::{command::Command, interaction::Interaction},
        gateway::{
            Ready,
            Activity
        },
        user::{
            OnlineStatus
        },
        Timestamp,
    },
    prelude::*,
};
use anyhow::Context as _;
use shuttle_secrets::SecretStore;

struct Bot;

#[async_trait]
impl EventHandler for Bot {
    async fn interaction_create(&self, ctx: Context, mut interaction: Interaction) {
        if let Interaction::ApplicationCommand(ref mut command) = interaction {
            println!("Perintah diterima: {:#?}", command);

            let _content = match command.data.name.as_str() {
                "perkenalan" => commands::perkenalan::run(interaction, &ctx).await,
                "perkenalan-slash" => commands::perkenalan_slash::run(interaction, &ctx).await,
                "kelas" => commands::kelas::run(interaction, &ctx).await,
                "bantuan" => commands::bantuan::run(interaction, &ctx).await,
                _ => {
                    let msg = command
                        .channel_id
                        .send_message(&ctx.http, |msg| {
                            msg.embed(|e| {
                                e.title("Terjadi kesalahan")
                                    .description("Tidak terimplementasi")
                                    .timestamp(Timestamp::now())
                            })
                        })
                        .await;

                    if let Err(why) = msg {
                        println!("Gagal mengirim pesan: {:?}", why);
                    }
                }
            };
        }
    }

    async fn ready(&self, ctx: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);

        let activity = Activity::listening("Mars SMAN 1 Kudus");

        ctx.set_presence(Some(activity), OnlineStatus::default()).await;

        let commands = Command::create_global_application_command(&ctx.http, |command| {
            commands::kelas::register(command)
        })
        .await;

        println!(
            "Slash Command diterima: {:#?}",
            commands
        );
    }
}

#[shuttle_runtime::main]
async fn serenity(
    #[shuttle_secrets::Secrets] secret_store: SecretStore
) -> shuttle_serenity::ShuttleSerenity {
    let token = secret_store.get("DISCORD_TOKEN").context("DISCORD_TOKEN was not found")?;

    let mut client = Client::builder(&token, GatewayIntents::empty())
        .event_handler(Bot)
        .await
        .expect("Terjadi kesalahan saat membuat klien");

    if let Err(why) = client.start().await {
        println!("Kesalahan memulai klien: {:#?}", why);
    }

    Ok(client.into())
}