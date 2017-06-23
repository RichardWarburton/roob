use irc::client::prelude::*;
use irc::client::prelude::Command::*;
use std::collections::HashMap;
use regex::Regex;

use plugins::*;

pub struct KarmaPlugin {
    regex : Regex,
    scores : HashMap<String, i32>,
}

impl KarmaPlugin {
    pub fn new() -> KarmaPlugin {
        KarmaPlugin {
            regex: Regex::new(r"([a-zA-Z0-9_]{2,})([\\+\\-]{2})").unwrap(),
            scores: HashMap::new(),
        }
    }
}

impl Plugin for KarmaPlugin {

    fn on_message(&mut self, server: &IrcServer, message: Message) {
        match message.command {
            PRIVMSG(channel, text) => {
                for cap in self.regex.captures_iter(&text) {
                    let item = &cap[1];
                    let mut score = self.scores.entry(String::from(item)).or_insert(0);

                    *score += if &cap[2] == "++" {1} else {-1};

                    server.send(Message{
                        tags: None,
                        prefix: Some(String::from("irc-rs")),
                        command: PRIVMSG(channel.clone(), format!("{} not has karma {}", item, score))
                    }).expect("Couldn't send message");
                }
            },
            _ => (),
        }
    }
}
