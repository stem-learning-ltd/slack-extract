use serde::Deserialize;
use std::fs::File;
use std::io::BufReader;
use std::error::Error;


#[derive(Deserialize, Debug)]
struct Message {
    text: String,
}

#[derive(Deserialize, Debug)]
struct SlackExport {
    messages: Vec<Message>,
}

fn main() -> Result<(), Box<dyn Error>> {
    let file = File::open("slack_export.json")?;
    let reader = BufReader::new(file);
//    let slack_export: SlackExport = serde_json::from_reader(reader)?;
    let messages: Vec<Message> = serde_json::from_reader(reader)?;

    for message in messages {
        println!("{}", message.text);
    }

    Ok(())
}
