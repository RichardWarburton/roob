
extern crate irc;

use std::default::Default;
use irc::client::prelude::*;

use irc::client::prelude::Command::*;

fn main() {
    let cfg = Config {
        nickname: Some(format!("irc-rs")),
        server: Some(format!("irc.uwcs.co.uk")),
        channels: Some(vec![format!("#roob")]),
        .. Default::default()
    };
    let server = IrcServer::from_config(cfg).unwrap();
    let mut plugins : Vec<Box<Plugin>> = Vec::new();
    let hwp : HelloWorldPlugin = HelloWorldPlugin{};
    plugins.push(Box::new(hwp));
    server.identify().unwrap();
    for message in server.iter() {
        match message {
            Err(e) => println!("Error: {:?}", e),
            Ok(msg) => handle_message(&plugins, &server, msg),
        }
    }
}

fn handle_message(plugins : &Vec<Box<Plugin>>, server : &IrcServer, message : Message) {
    // TODO: better logging
    println!("{:?}", message);

    for plugin_box in plugins {
        let plugin :&Plugin = plugin_box.as_ref();
        plugin.on_message(server, message.clone());
    }
}

trait Plugin {
    fn on_message(&self, server : &IrcServer, message : Message);
}

pub struct HelloWorldPlugin {

}

impl Plugin for HelloWorldPlugin {
    fn on_message(&self, server: &IrcServer, message: Message) {
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
