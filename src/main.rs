
extern crate irc;
extern crate regex;

use irc::client::prelude::*;

mod plugins;
mod join_plugin;
mod karma_plugin;
mod hello_world_plugin;

use plugins::*;

fn main() {
    let server = IrcServer::new("config.json").unwrap();
    let mut plugins : Vec<Box<Plugin>> = Vec::new();
    plugins.push(Box::new(hello_world_plugin::HelloWorldPlugin{}));
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
