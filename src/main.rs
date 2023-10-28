use serde::Deserialize;
use std::env;
use std::fs::{File, OpenOptions};
use std::io::{BufReader, Write, BufWriter};
use std::error::Error;
use walkdir::WalkDir;

#[derive(Deserialize, Debug)]
struct Message {
    text: String,
}


fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage: {} <directory>", args[0]);
        return Ok(());
    }
    
    let directory = &args[1];

    let output_file_path = format!("{}.txt", directory);
    let output_file = OpenOptions::new().create(true).write(true).truncate(true).open(&output_file_path)?;
    let mut writer = BufWriter::new(output_file);


    for entry in WalkDir::new(directory) {
        let entry = entry?;
        println!("{}", entry.path().display());
        if entry.file_type().is_file() && entry.path().extension().unwrap() == "json" {
            println!("is json file");
            let file = File::open(entry.path())?;
            let reader = BufReader::new(file);
            
            match serde_json::from_reader::<_, Vec<Message>>(reader) {
                Ok(messages) => {
                    for message in messages {
                        writeln!(&mut writer, "{}", message.text)?;
                    }
                }
                Err(e) => {
                    eprintln!("Error: {}", e);
                }
            }
        }
    }



    Ok(())
}
