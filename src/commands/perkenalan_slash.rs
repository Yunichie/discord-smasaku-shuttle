use crate::capitalizer::capitalize;

use serenity::{
    builder::{
        CreateApplicationCommand
    },
    model::{
        prelude::{
            command::{
                CommandOptionType
            },
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
use regex::Regex;
//use tokio::fs::File;

pub fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    command
        .name("perkenalan-slash")
        .description("Perkenalkan diri kamu!")
        .create_option(|option| {
            option
                .name("nama")
                .description("Masukkan nama kamu!")
                .kind(CommandOptionType::String)
                .required(true)
        })
        .create_option(|option| {
            option
                .name("kelas")
                .description("Kelas berapa kamu?")
                .kind(CommandOptionType::String)
                .required(true)
        })
        .create_option(|option| {
            option
                .name("angkatan")
                .description("Angkatan tahun berapa kamu?")
                .kind(CommandOptionType::String)
                .required(true)
        })
        .create_option(|option| {
            option
                .name("medsos")
                .description("Media sosial kamu! (opsional)")
                .kind(CommandOptionType::String)
                .required(false)
        })
}

pub async fn run(interaction: Interaction, ctx: &Context) {
    let mut options = interaction.application_command().unwrap();
    let nama = options
        .data
        .options
        .iter()
        .find(|option| option.name == "nama")
        .cloned();

    let kelas = options
        .data
        .options
        .iter()
        .find(|option| option.name == "kelas")
        .cloned();

    let angkatan = options
        .data
        .options
        .iter()
        .find(|option| option.name == "angkatan")
        .cloned();

    let medsos_opsi = options
        .data
        .options
        .iter()
        .find(|option| option.name == "medsos")
        .cloned();

    let nama = nama.unwrap().value.unwrap().as_str().unwrap().to_owned();
    let kelas = kelas.unwrap().value.unwrap().as_str().unwrap().to_owned();
    let angkatan = angkatan.unwrap().value.unwrap().as_str().unwrap().to_owned();
    let mut medsos = String::new();

    if medsos_opsi.is_none() {
        medsos.push_str("-");
    } else {
        medsos.push_str(medsos_opsi.unwrap().value.unwrap().as_str().unwrap());
    }

    //let cache = &ctx.cache;
    let user = &options.user;
    let roles = &options.member.as_ref().unwrap().roles;
    let role_id = &895264956751163412;  // id role smasaku
    let ch_id = &895265138565865502;    // id channel #introduction

    // User sudah mempunyai role smasaku
    if roles.iter().any(|&i| i.as_u64() == role_id) {
        let sudah_punya_role = options
            .create_interaction_response(&ctx.http, |resp| {
                resp.kind(InteractionResponseType::ChannelMessageWithSource)
                    .interaction_response_data(|msg| {
                        msg.content("Kamu sudah memperkenalkan diri!")
                    })
            })
            .await;

        if let Err(why) = sudah_punya_role {
            println!("Terjadi kesalahan (sudah_punya_role) saat mengirim pesan: {:?}", why);
        }
    }

    // User belum mempunyai role smasaku
    if !roles.iter().any(|&i| i.as_u64() == role_id) && options.channel_id.as_u64() != ch_id {
        let ch_err = options
            .create_interaction_response(&ctx.http, |resp| {
                resp.kind(InteractionResponseType::ChannelMessageWithSource)
                    .interaction_response_data(|msg| {
                        msg.content("Kamu hanya bisa memperkenalkan diri di <#895265138565865502>!")
                    })
            })
            .await;

        if let Err(why) = ch_err {
            println!("Terjadi kesalahan (ch_err) saat mengirim pesan: {:?}", why);
        }
    }

    if !roles.iter().any(|&i| i.as_u64() == role_id) && options.channel_id.as_u64() == ch_id {
        // RegEx
        let xi_xii = Regex::new(r"^(XI|XII)\s(MIPA|IPS)\s([0-9]|1[0-2])$").unwrap();
        let x = Regex::new(r"^(X)\s([0-9]|1[0-2])$").unwrap();
        let regex_angkatan = Regex::new(r#"^(([0-9]){4})|(^([0-9]){4}/([0-9]){4})$"#).unwrap();

        if !(x.is_match(&kelas.to_uppercase())) && !(xi_xii.is_match(&kelas.to_uppercase())) {
            // Kirim embed error!
            let kls_err = options
                .create_interaction_response(&ctx.http, |resp| {
                    resp.kind(InteractionResponseType::ChannelMessageWithSource)
                        .interaction_response_data(|msg| {
                            msg.embed(|e| {
                                e.color((247, 10, 10))
                                    .title("Format perkenalan tidak sesuai!")
                                    .description(
                                        r#"
Format Kelas yang benar:
**[XI/XII] [MIPA/IPS] [1-12]**;
**X [1-12]**.

Contoh: X 3, XI MIPA 3
                    "#
                                    )
                                    .footer(|f| f.text("[] tidak perlu dimasukkan."))
                            })
                        })
                })
                .await;

            if let Err(why) = kls_err {
                println!("Terjadi kesalahan (kls_err) saat mengirim pesan: {:?}", why);
            }
        } else if !(regex_angkatan.is_match(&angkatan)) {
            // Kirim embed error!
            let angkt_err = options
                .create_interaction_response(&ctx.http, |resp| {
                    resp.kind(InteractionResponseType::ChannelMessageWithSource)
                        .interaction_response_data(|msg| {
                            msg.embed(|e| {
                                e.color((247, 10, 10))
                                    .title("Format perkenalan tidak sesuai!")
                                    .description(
                                        r#"
Format Angkatan yang benar:
1. **[tahun]/[tahun]**;
2. **[tahun masuk]**.

Contoh: 2021/2022 atau cukup 2021.
                    "#
                                    )
                                    .footer(|f| f.text("[] tidak perlu dimasukkan."))
                            })
                        })
                })
                .await;

            if let Err(why) = angkt_err {
                println!("Terjadi kesalahan  (angkt_err) saat mengirim pesan: {:?}", why);
            }
        } else {
            // TODO: Gunakan file!
            // Kirim embed berisi data (perkenalan) yang dimasukkan
            let perkenalan_slash = options
                .create_interaction_response(&ctx.http, |resp| {
                    resp.kind(InteractionResponseType::ChannelMessageWithSource)
                        .interaction_response_data(|msg| {
                            msg.embed(|e| {
                                e.color((247, 10, 10))
                                    .title("Perkenalan")
                                    .fields(vec![
                                        ("Nama", capitalize::capitalize(&nama), false),
                                        ("Kelas", kelas.to_uppercase(), false),
                                        ("Angkatan", angkatan, false),
                                        ("Media Sosial", medsos, false),
                                    ])
                                    .thumbnail(&user.avatar_url().unwrap())
                                    .image("https://media.discordapp.net/attachments/895265138565865502/1019552086444232755/welcome.jpg")
                                    .footer(|f| f.text(&user.tag()))
                                    .timestamp(Timestamp::now())
                                })
                            })
                        })
                .await;

            // Follow-up untuk mengambil role kelas
            let _followup = options
        .create_followup_message(&ctx.http, |resp| {
            resp.content(format!("<@{}> Jangan lupa untuk mengambil _roles_ di <#881539526227537950> setelah memperkenalkan diri.", options.user.id.as_u64()))
        }).await;

            if let Err(why) = perkenalan_slash {
                println!("Terjadi kesalahan (perkenalan_slash) saat mengirim pesan: {:?}", why);

                let _kesalahan = options
                .create_interaction_response(&ctx.http, |resp| {
                    resp.kind(InteractionResponseType::ChannelMessageWithSource)
                        .interaction_response_data(|msg| {
                            msg.ephemeral(true);
                            msg.content("Terjadi kesalahan! Harap jalankan ulang perintah.")
                        })  
                }).await;
            } else {
                // Beri role smasaku
                let _add_smasaku = options
                    .member
                    .as_mut()
                    .unwrap()
                    .add_role(&ctx.http, *role_id)
                    .await;
            }
        }
    }
}