extern crate irc;
extern crate modules;
extern crate regex;

use irc::client::prelude::*;
use irc::client::prelude::Command::*;
use std::collections::HashMap;
use std::any::Any;
use std::ops::DerefMut;
use std::ops::Deref;
use regex::Regex;
use std::mem;

use modules::*;

plugin! ();

struct State {
    regex : Regex,
    scores : HashMap<String, i32>,
}

fn init(server: &IrcServer) -> State {
    State {
        regex: Regex::new(r"([a-zA-Z0-9_]{2,})([\\+\\-]{2})").unwrap(),
        scores: HashMap::new(),
    }
}

fn on_message(context : &mut State, server: &IrcServer, message: Message) {
    match message.command {
        PRIVMSG(channel, text) => {
            match parse_command(text.clone()) {
                Args(cmd, arg) => {
                    if cmd == "karma" {
                        let score = context.scores[&arg];
                        server.send_privmsg(
                            &channel,
                            &format!("{} has karma {}", arg, score)).unwrap();
                    }
                },
                Cmd(_) => (),
                Other => {
                    for cap in context.regex.captures_iter(&text) {
                        let item = &cap[1];
                        let mut score = context.scores.entry(String::from(item)).or_insert(0);

                        *score += if &cap[2] == "++" { 1 } else { -1 };

                        server.send_privmsg(
                            &channel,
                            &format!("{} not has karma {}", item, score)).unwrap();
                    }
                },
            }
        },
        _ => (),
    }
}
