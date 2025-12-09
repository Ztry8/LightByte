use chacha20poly1305::{
    ChaCha20Poly1305, Key, Nonce,
    aead::{Aead, KeyInit},
};
use rand::{TryRngCore, rngs::OsRng};
use std::{
    fs::{self, File},
    io::{BufWriter, Read, Write},
};
use walkdir::WalkDir;

macro_rules! error {
    ($msg:expr) => {{
        println!("{}\n\nThere are two operations:\n\n1) lightbyte key\nGenerates a key file used for archive encryption.\n", $msg);
        println!("2) lightbyte compress <folder-name>\nCreates and encrypts a .pak archive using the key file.\n");
        std::process::exit(0);
    }};
}

macro_rules! check {
    ($cond:expr, $msg:expr) => {
        if !$cond {
            error!($msg);
        }
    };
}

fn create_archive(input_dir: &str) {
    let key_bytes: Vec<u8> = fs::read_to_string("key")
        .unwrap_or_else(|_| error!("You need to create the file named \'key\' containing 32-bit key. You can generate a new key using: lightbyte key"))
        .lines()
        .map(|line| line.trim().parse::<u8>().unwrap())
        .collect();

    check!(
        key_bytes.len() == 32,
        "Your key must be 32-bit! You can generate a new key using: lightbyte key"
    );

    println!("Compressing files...");

    let mut archive: Vec<u8> = Vec::new();

    let mut files = Vec::new();
    for entry in WalkDir::new(input_dir) {
        let entry = entry.unwrap_or_else(|_| {
            println!("\nfolder {} does not exist!\n", input_dir);
            std::process::exit(0);
        });

        if entry.file_type().is_file() {
            files.push(entry.path().to_str().unwrap().to_owned());
        }
    }

    archive.extend(&(files.len() as u32).to_le_bytes());

    for path in &files {
        let mut data = Vec::new();
        File::open(path).unwrap().read_to_end(&mut data).unwrap();

        let name = path.as_bytes();

        archive.extend(&(name.len() as u16).to_le_bytes());
        archive.extend(name);

        archive.extend(&(data.len() as u64).to_le_bytes());
        archive.extend(&data);
    }

    let compressed = zstd::encode_all(&archive[..], 3).unwrap();
    println!("Encrypting archive...");

    let cipher = ChaCha20Poly1305::new(Key::from_slice(&key_bytes));

    let mut nonce_bytes = [0u8; 12];
    OsRng.try_fill_bytes(&mut nonce_bytes).unwrap();

    let nonce = Nonce::from_slice(&nonce_bytes);
    let encrypted = cipher.encrypt(&nonce, compressed.as_ref()).unwrap();

    let mut out = File::create(format!("{}.pak", input_dir)).unwrap();
    out.write_all(&nonce_bytes).unwrap();
    out.write_all(&encrypted).unwrap();

    println!("Archive created successfully!\n");
}

fn create_key() {
    println!("Generating key...");

    let mut lines = [0u8; 32];
    OsRng.try_fill_bytes(&mut lines).unwrap();

    let file = File::create("key").unwrap();
    let mut writer = BufWriter::new(file);

    for line in lines {
        writeln!(writer, "{}", line).unwrap();
    }

    println!("Key generated successfully!\n");
}

fn main() {
    println!(
        "\nLightByte v{} by Ztry8 (AslanD)\nUtility to compress and encrypt game assets\n",
        env!("CARGO_PKG_VERSION")
    );

    let args: Vec<String> = std::env::args().collect();
    check!(args.len() > 1, "You need to provide at least one argument!");

    match args[1].to_lowercase().as_str() {
        "help" => error!("Help:"),
        "key" => create_key(),
        "compress" => {
            check!(
                args.len() == 3,
                "You need to specify the name of the folder to pack."
            );

            create_archive(&args[2]);
        }
        _ => error!("Unknown operation!"),
    }
}
