use chrono::prelude::*;
use std::collections::LinkedList;

#[derive(Clone)]
pub struct MessageObject {
  pub text: String,
  pub channel: String,
  pub user: String,
  pub date: DateTime<Local>,
  pub command: bool,
  pub command_txt: String,
  pub command_args: LinkedList<String>
}

impl Default for MessageObject {
  fn default() -> MessageObject {
    MessageObject {
      text: String::from(""),
      user: String::from(""),
      channel: String::from(""),
      date: Local::now(),
      command: false,
      command_txt: String::from(""),
      command_args: LinkedList::new()
    }
  }
}