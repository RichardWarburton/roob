extern crate irc;
extern crate modules;
extern crate regex;

use irc::client::prelude::*;
use irc::client::prelude::Command::*;
use std::collections::HashMap;
use regex::Regex;

use modules::*;

pub struct State {
    regex : Regex,
    scores : HashMap<String, i32>,
}

#[no_mangle]
pub fn init_state() -> Box<State> {
    Box::new(State{
        regex: Regex::new(r"([a-zA-Z0-9_]{2,})([\\+\\-]{2})").unwrap(),
        scores: HashMap::new(),
    })
}

#[no_mangle]
pub fn on_message(server: &IrcServer, message: Message) {
    /*match message.command {
        PRIVMSG(channel, text) => {
            match parse_command(text.clone()) {
                Args(cmd, arg) => {
                    if cmd == "karma" {
                        let score = self.scores[&arg];
                        server.send_privmsg(
                            &channel,
                            &format!("{} has karma {}", arg, score)).unwrap();
                    }
                },
                Cmd(_) => (),
                Other => {
                    for cap in self.regex.captures_iter(&text) {
                        let item = &cap[1];
                        let mut score = self.scores.entry(String::from(item)).or_insert(0);

                        *score += if &cap[2] == "++" {1} else {-1};

                        server.send_privmsg(
                            &channel,
                            &format!("{} not has karma {}", item, score)).unwrap();
                    }
                },
            }
        },
        _ => (),
    }*/
}
