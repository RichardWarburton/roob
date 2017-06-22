
extern crate irc;
extern crate regex;

use std::default::Default;
use irc::client::prelude::*;
use irc::client::prelude::Command::*;

const COMMAND_PREFIX : &str = "!";

mod join_plugin;
mod karma_plugin;

fn main() {
    let cfg = Config {
        nickname: Some(format!("irc-rs")),
        server: Some(format!("irc.uwcs.co.uk")),
        channels: Some(vec![format!("#roob")]),
        .. Default::default()
    };
    let server = IrcServer::from_config(cfg).unwrap();
    let mut plugins : Vec<Box<Plugin>> = Vec::new();
    plugins.push(Box::new(HelloWorldPlugin{}));
    plugins.push(Box::new(karma_plugin::KarmaPlugin::new()));
    plugins.push(Box::new(join_plugin::JoinPlugin{}));
    server.identify().unwrap();
    for message in server.iter() {
        match message {
            Err(e) => println!("Error: {:?}", e),
            Ok(msg) => handle_message(&mut plugins, &server, msg),
        }
    }
}

fn handle_message(plugins : &mut Vec<Box<Plugin>>, server : &IrcServer, message : Message) {
    // TODO: better logging
    println!("{:?}", message);

    for plugin in plugins {
        plugin.on_message(server, message.clone());
    }
}

pub fn parse_command(text : String) -> Option<String> {
    if text.starts_with(COMMAND_PREFIX) {
        let end = text.find(' ').unwrap_or(text.len());
        Some(String::from(&text[COMMAND_PREFIX.len() .. end ]))
    } else {
        None
    }
}

pub trait Plugin {
    fn on_message(&mut self, server : &IrcServer, message : Message);
}

struct HelloWorldPlugin {

}

impl Plugin for HelloWorldPlugin {
    fn on_message(&mut self, server: &IrcServer, message: Message) {
        match message.command {
            PRIVMSG(channel, text) => {
                if text == "Hi" {
                    server.send(Message{
                        tags: None,
                        prefix: Some(String::from("irc-rs")),
                        command: PRIVMSG(channel, String::from("lo!"))
                    }).expect("Couldn't send message");
                }
            },
            _ => (),
        }
    }
}
