use crate::capitalizer::capitalize;

use serenity::{
    builder::{
        CreateApplicationCommand
    },
    collector::{
        modal_interaction_collector::{
            CollectModalInteraction
        }
    },
    model::{
        prelude::{
            application::{
                interaction::{
                    Interaction,
                    InteractionResponseType
                },
                component::{
                    InputTextStyle,
                    ActionRowComponent
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
use std::time::Duration;

pub fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    command
        .name("perkenalan")
        .description("Perkenalkan diri kamu... menggunakan Interaksi Modal!")
}

pub async fn run(interaction: Interaction, ctx: &Context) {
    let mut command = interaction.to_owned().application_command().unwrap();
    let user = &command.user;
    let roles = &command.member.as_ref().unwrap().roles;
    let role_id = &895264956751163412; // id role smasaku
    let ch_id = &895265138565865502; // id channel #introduction

    // User sudah mempunyai role smasaku
    if roles.iter().any(|&i| i.as_u64() == role_id) {
        let sudah_punya_role = command
            .create_interaction_response(&ctx.http, |resp| {
                resp.kind(InteractionResponseType::ChannelMessageWithSource)
                    .interaction_response_data(|msg| msg.content("Kamu sudah memperkenalkan diri!"))
            })
            .await;

        if let Err(why) = sudah_punya_role {
            println!("Terjadi kesalahan (sudah_punya_role) saat mengirim pesan: {:?}", why);
        }
    }

    // User belum mempunyai role smasaku
    if !roles.iter().any(|&i| i.as_u64() == role_id) && command.channel_id.as_u64() != ch_id {
        let ch_err = command
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

    if !roles.iter().any(|&i| i.as_u64() == role_id) && command.channel_id.as_u64() == ch_id {
        let modal = interaction
            .to_owned()
            .application_command()
            .unwrap()
            .create_interaction_response(&ctx.http, |resp| {
                resp.kind(InteractionResponseType::Modal)
                    .interaction_response_data(|response| {
                        response.custom_id("perkenalan");
                        response.title("Perkenalan");
                        response.components(|act_row| {
                            act_row.create_action_row(|modal| {
                                modal.create_input_text(|input| {
                                    input.custom_id("perkenalan_nama");
                                    input.style(InputTextStyle::Short);
                                    input.label("Nama");
                                    input.placeholder("Masukkan nama kamu!");
                                    input.required(true)
                                })
                            });
                            act_row.create_action_row(|modal| {
                                modal.create_input_text(|input| {
                                    input.custom_id("perkenalan_kelas");
                                    input.style(InputTextStyle::Short);
                                    input.label("Kelas");
                                    input.placeholder("Masukkan kelas kamu!");
                                    input.required(true)
                                })
                            });
                            act_row.create_action_row(|modal| {
                                modal.create_input_text(|input| {
                                    input.custom_id("perkenalan_angkatan");
                                    input.style(InputTextStyle::Short);
                                    input.label("Angkatan");
                                    input.placeholder("Angkatan tahun berapa kamu?");
                                    input.required(true)
                                })
                            });
                            act_row.create_action_row(|modal| {
                                modal.create_input_text(|input| {
                                    input.custom_id("perkenalan_medsos");
                                    input.style(InputTextStyle::Short);
                                    input.label("Media Sosial");
                                    input.placeholder("Media sosial kamu! (opsional)");
                                    input.required(false)
                                })
                            })
                        })
                    })
            })
            .await;

        if let Err(why) = modal {
            println!("Terjadi kesalahan (modal) saat mengirim pesan: {:?}", why);
        }

        let response = CollectModalInteraction::new(&ctx.shard)
            .author_id(
                interaction
                    .to_owned()
                    .application_command()
                    .unwrap()
                    .user
                    .id
            )
            .timeout(Duration::from_secs(3600))
            .await
            .unwrap();

        let collected = response
            .data
            .components
            .to_owned()
            .into_iter()
            .flat_map(|x| x.to_owned().components)
            .collect::<Vec<ActionRowComponent>>();

        let data = collected
            .to_owned()
            .iter()
            .map(|x| match x {
                ActionRowComponent::InputText(inp) => {
                    if inp.to_owned().value == "" {
                        return "-".to_string();
                    } else {
                        inp.to_owned().value
                    }
                }
                _ => format!("Tidak ditemukan!")
            })
            .collect::<Vec<String>>();
        // data[0] = nama
        // data[1] = kelas
        // data[2] = angkatan
        // data[3] = medsos

        let xi_xii = Regex::new(r"^(XI|XII)\s(MIPA|IPS)\s([0-9]|1[0-2])$").unwrap();
        let x = Regex::new(r"^(X)\s([0-9]|1[0-2])$").unwrap();
        let regex_angkatan = Regex::new(r#"^(([0-9]){4})|(^([0-9]){4}/([0-9]){4})$"#).unwrap();

        if !(x.is_match(&data[1].to_uppercase())) && !(xi_xii.is_match(&data[1].to_uppercase())) {
            // Kirim embed error!
            let kls_err = response
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
        } else if !(regex_angkatan.is_match(&data[2])) {
            // Kirim embed error!
            let angkt_err = response
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
                println!("Terjadi kesalahan (angkt_err) saat mengirim pesan: {:?}", why);
            }
        } else {
            // TODO: Gunakan file!
            // Kirim embed berisi data (perkenalan) yang dimasukkan
            let perkenalan_slash = response
                .create_interaction_response(&ctx.http, |resp| {
                    resp.kind(InteractionResponseType::ChannelMessageWithSource)
                        .interaction_response_data(|msg| {
                            msg.embed(|e| {
                                e.color((247, 10, 10))
                                    .title("Perkenalan")
                                    .fields(vec![
                                        ("Nama", capitalize::capitalize(&data[0]), false),
                                        ("Kelas", data[1].to_uppercase(), false),
                                        ("Angkatan", data[2].to_owned(), false),
                                        ("Media Sosial", data[3].to_owned(), false),
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
            let _followup = command
        .create_followup_message(&ctx.http, |resp| {
            resp.content(format!("<@{}> Jangan lupa untuk mengambil _roles_ di <#881539526227537950> setelah memperkenalkan diri.", command.user.id.as_u64()))
        }).await;

            if let Err(why) = perkenalan_slash {
                println!("Terjadi kesalahan (perkenalan_slash) saat mengirim pesan: {:?}", why);

                let _kesalahan = command
                    .create_interaction_response(&ctx.http, |resp| {
                        resp.kind(InteractionResponseType::ChannelMessageWithSource)
                            .interaction_response_data(|msg| {
                                msg.ephemeral(true);
                                msg.content("Terjadi kesalahan! Harap jalankan ulang perintah.")
                            })
                    })
                    .await;
            } else {
                // Beri role smasaku
                let _add_smasaku = command
                    .member
                    .as_mut()
                    .unwrap()
                    .add_role(&ctx.http, *role_id)
                    .await;
            }
        }
    }
}