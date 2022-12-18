use serenity::{
    builder::{
        CreateApplicationCommand
    },
    model::{
        prelude::{
            application::{
                interaction::{
                    Interaction,
                    InteractionResponseType
                }
            },
        },
        Timestamp
    },
    prelude::{
        *
    }
};

pub fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    command.name("kelas").description("Perbarui role kelas")
}

pub async fn run(interaction: Interaction, ctx: &Context) {
    let command = interaction.application_command().unwrap();
    let role_id = 1025826518259224608;
    //let xii = 881554418208739401;
    //let xi = 881554373719781376;
    //let x = 881554292736163910;

    if role_id == 1025826518259224608 {
        let edit_role = &command
            .guild_id
            .unwrap()
            .edit_role(&ctx.http, role_id, |r| r.name("icikwir 2"))
            .await;

        if let Err(why) = edit_role {
            println!("Terjadi kesalahan (edit_role): {:?}", why);
        }
    }

    let res = command
        .create_interaction_response(&ctx.http, |msg| {
            msg.kind(InteractionResponseType::ChannelMessageWithSource)
                .interaction_response_data(|d| d.content("kelas berhasil diperbarui."))
        })
        .await;

    if let Err(why) = res {
        println!("Terjadi kesalahan (res) saat mengirim pesan: {:?}", why);
    }
}