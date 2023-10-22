use rand::Rng;
use std::fs::File;
use std::io::{self, Write};

fn main() -> io::Result<()> {
    // Jumlah data yang akan di-generate
    let jumlah_data = 1_000_000;

    // Buat file untuk menyimpan perintah SQL
    let mut file = File::create("data.sql")?;

    // Buat loop untuk menghasilkan perintah SQL
    for _ in 0..jumlah_data {
        let name = generate_random_string(10);
        let email = generate_random_number(18, 65);
        // let email = generate_random_string(10);

        let sql_command = format!(
            "INSERT INTO sellers (name, email) VALUES ('{}', '{}@gmail.com');",
            name, email
        );

        // Tulis perintah SQL ke dalam file
        writeln!(file, "{}", sql_command)?;
    }

    println!("Data SQL telah di-generate dan disimpan dalam data.sql");

    Ok(())
}

// Fungsi untuk menghasilkan string acak
fn generate_random_string(length: usize) -> String {
    let mut rng = rand::thread_rng();
    let letters = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ";
    let random_string: String = (0..length)
        .map(|_| rng.gen_range(0..letters.len()))
        .map(|i| letters.chars().nth(i).unwrap())
        .collect();
    random_string
}

// Fungsi untuk menghasilkan angka acak
fn generate_random_number(min: i32, max: i32) -> i32 {
    let mut rng = rand::thread_rng();
    rng.gen_range(min..=max)
}
