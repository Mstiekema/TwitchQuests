use futures::prelude::*;
use irc::client::prelude::*;
use std::collections::LinkedList;

mod structs;

#[tokio::main]
async fn main() -> irc::error::Result<()> {
  // Handle connection to chat
  println!("[DEBUG] Connecting to Twitch Chat...");
  let mut client = Client::new("config.toml").await?;
  client.identify()?;
  let mut stream = client.stream()?;
  let sender = client.sender();
  println!("[DEBUG] Connected to Twitch Chat");

  // Handle all messages
  while let Some(message) = stream.next().await.transpose()? {
    match message.command {
      Command::PRIVMSG(ref target, ref txt) => {
        let msg_obj = handle_messages(message.clone(), target, txt);

        if msg_obj.command {
          handle_command(msg_obj.clone(), sender.clone());
        }

        println!("[{}] ({}) {}: {}", msg_obj.date.format("%d/%m/%Y %T"), msg_obj.channel, msg_obj.user, msg_obj.text);
      }
      _ => ()
    }
  }

  Ok(())
}

// Handle commands calls
fn handle_command(msg: structs::messages::MessageObject, sender: irc::client::Sender) {
  match msg.command_txt.as_ref() {
    "!test" => sender.send_privmsg(msg.channel, "Testing 123 MrDestructoid").unwrap(),
    _ => sender.send_privmsg(msg.channel, "A command was used :)").unwrap()
  }
}

// Assign the message to the custom message object
fn handle_messages(msg: irc::client::prelude::Message, target: &String, txt: &String) -> structs::messages::MessageObject {
  let mut message = structs::messages::MessageObject {
    text: txt.to_string(),
    user: msg.source_nickname().unwrap().replace("#", "").to_string(),
    channel: target.to_string(),
    ..Default::default()
  };

  if txt.contains("!") {
    let mut message_array = txt.split_whitespace();
    message.command = true;
    message.command_txt = message_array.next().unwrap().to_string();
    message.command_args = message_array.map(|word| word.to_string()).collect::<LinkedList<String>>();
  }
  
  return message;
}