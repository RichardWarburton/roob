
extern crate irc;
extern crate regex;
extern crate modules;

use irc::client::prelude::*;

mod join_plugin;
mod karma_plugin;

use modules::*;

extern crate libloading as lib;

use lib::{Library, Symbol};

struct DynPlugin {
    lib : Library,
}

impl DynPlugin {
    fn on_message(&mut self, server: &IrcServer, message: Message) -> () {
        unsafe {
            let on_message: Symbol<extern fn(server: &IrcServer, message: Message) -> ()> =
                self.lib.get(b"on_message").unwrap();

            on_message(server, message);
        }
    }
}

// DynPlugin
fn load_plugins() -> DynPlugin {
    DynPlugin {
        lib : Library::new("libs/plugins/target/debug/plugins.dll").unwrap(),
    }
}

fn main() {
    let plugin = load_plugins();
    let server = IrcServer::new("config.json").unwrap();
    let mut plugins : Vec<DynPlugin> = Vec::new();
    plugins.push(plugin);
    server.identify().unwrap();
    for message in server.iter() {
        match message {
            Err(e) => println!("Error: {:?}", e),
            Ok(msg) => handle_message(&mut plugins, &server, msg),
        }
    }
}

fn handle_message(plugins : &mut Vec<DynPlugin>, server : &IrcServer, message : Message) {
    // TODO: better logging
    println!("{:?}", message);

    for plugin in plugins {
        plugin.on_message(server, message.clone());
    }
}
