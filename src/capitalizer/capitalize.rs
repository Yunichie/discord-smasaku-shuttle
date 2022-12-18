use regex::Regex;

pub fn capitalize(string: &str) -> String {
    let s = string.to_lowercase(); // Ubah argumen menjadi lowercase.
    let s_split = &s.split_whitespace().collect::<Vec<&str>>(); // Pisah argumen berdasarkan spasi, masukkan ke array.
    let find_first_letter = Regex::new(r"\b(\w)").unwrap(); // "Take every character after the word boundary"
    let mut return_this = String::new(); // String kosong untuk menyimpan string yang sudah diubah.

    for i in s_split {
        // Lakukan perulangan terhadap array
        return_this.push_str(
            &find_first_letter
                .replace(i, i.chars().next().unwrap().to_uppercase().to_string())
                .to_string()
                .chars()
                .collect::<String>(),
        ); // Masukkan karakter yang (akan) sudah dikapitalkan ke argumen asli.
        return_this.push_str(" "); // Masukkan spasi
    }
    return_this.pop(); // Perulangan akan menyebabkan argumen mempunyai spasi ekstra. Hapus.

    return return_this; // Hasil akhir.
}