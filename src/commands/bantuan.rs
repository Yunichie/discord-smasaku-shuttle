use serenity::{
    builder::{
        CreateApplicationCommand
    },
    model::{
        application::{
            interaction::{
                Interaction,
                InteractionResponseType,
            }
        },
        Timestamp,
    },
    prelude::*,
};

pub fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    command.name("bantuan").description("Dapatkan bantuan!")
}

pub async fn run(interaction: Interaction, ctx: &Context) {
    let interaction = interaction.application_command().unwrap();
    let user = &interaction.user;

    let bantuan = interaction
    .create_interaction_response(&ctx.http, |resp| {
        resp
        .kind(InteractionResponseType::ChannelMessageWithSource)
        .interaction_response_data(|msg| {
            msg.embed(|e| {
                e
                .color((247, 10, 10))
                .title("Bantuan")
                .fields(vec![
                    ("Nama", "`/perkenalan`, `/perkenalan-slash`", false),
                    ("Penggunaan", r#"
/perkenalan
/perkenalan-slash `nama:` `kelas:` `angkatan:`
                    "#, false),
                    ("Format", r#"
(`/perkenalan` dan `/perkenalan-slash`).
`nama`: `nama kamu`
**Karakter Aa-Zz**
          
`kelas`: `kelas kamu`
**[XI/XII] [MIPA/IPS] [1-12]**,
**[X] [1-12]**
`angkatan`: `tahun masuk`
**4 digit angka**;
**4 digit angka/4 digit angka**
                    "#, false),
                    ("Contoh", r#"
(`/perkenalan-slash`)
1. /perkenalan-slash `nama: Nama Saya` `kelas: X 3` `angkatan: 2021`
2. /perkenalan-slash `nama: Nama Saya` `kelas: XI MIPA 3` `angkatan: 2021/2022` `medsos: @medsos_saya`
                    "#, false),
                    ("Catatan", r#"
(`/perkenalan` dan `/perkenalan-slash`)
1. Tidak perlu memasukkan tanda [] untuk `kelas`.
2. Gunakan hanya salah satu format dari dua untuk `angkatan`.
3. Tidak perlu memasukkan **spasi** setelah menekan/memilih opsi yang hendak diisi.
4. Tidak perlu memasukkan **spasi** setelah selesai memasukkan `nama`, `kelas`, dan `angkatan`; langsung tekan opsi yang diperlukan.
5. Opsi `medsos` adalah opsional; tidak wajib diisi.
                    "#, false)
                ])
                .thumbnail(ctx.cache.current_user().avatar_url().unwrap())
                .image("https://media.discordapp.net/attachments/1024284784077320255/1032675495575302204/unknown.png")
                .image("https://media.discordapp.net/attachments/1024284784077320255/1039201766589337630/perkenalan-modal.png?width=400&height=480")
                .footer(|f| f.icon_url(&user.avatar_url().unwrap()).text(&user.tag()))
                .timestamp(Timestamp::now())
            })
        })
    }).await;

    if let Err(why) = bantuan {
        println!("Terjadi kesalahan saat mengirim pesan: {:?}", why);
    }
}