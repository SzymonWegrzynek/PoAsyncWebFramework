use std::{error::Error, io};

use crate::{models::payload::Payload, services::send::Send};

pub struct Cli;

impl Cli {
    pub async fn run() -> Result<(), Box<dyn Error>> {
        println!("Wprowadź komendę: program -rc <tekst>/-rf <sciezka> -l <jezyk>:");

        loop {
            let mut input = String::new();

            io::stdin().read_line(&mut input)?;

            let input = input.trim();

            if input == "exit" {
                break;
            }

            let args: Vec<&str> = input.split_whitespace().collect();

            match args.as_slice() {
                ["program", "-rf", file_path, "-l", language] => {
                    println!("Odczytywanie tekst z pliku");
                    let payload = Payload::File(file_path.to_string());
                    Send::send_speech(payload, language.to_string()).await?;
                }

                ["program", "-rc", text, "-l", language] => {
                    println!("Odczytywanie tekst z konsoli");
                    let payload = Payload::Text(text.to_string());
                    Send::send_speech(payload, language.to_string()).await?;
                }

                _ => {
                    println!("Błąd w komendzie");
                }
            }
        }

        Ok(())
    }
}
