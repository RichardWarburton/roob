
extern crate irc;
extern crate regex;
extern crate modules;
extern crate libloading as lib;

mod karma_plugin;
mod horrible_perl_script;

use irc::client::prelude::*;
use lib::{Library, Symbol};
use horrible_perl_script::setup_plugins;

fn main() {
    let plugin_lib_paths = setup_plugins();
    start_bot(plugin_lib_paths);
}

fn start_bot(plugin_lib_paths : Vec<String>) {
    let mut plugins: Vec<Plugin> = Vec::new();
    for lib_path in plugin_lib_paths {
        plugins.push(load_plugin(lib_path));
    }

    let server = IrcServer::new("config.json").unwrap();
    server.identify().unwrap();
    for message in server.iter() {
        match message {
            Err(e) => println!("Error: {:?}", e),
            Ok(msg) => handle_message(&mut plugins, &server, msg),
        }
    }
}

fn handle_message(plugins : &mut Vec<Plugin>, server : &IrcServer, message : Message) {
    // TODO: better logging
    println!("{:?}", message);

    for plugin in plugins {
        plugin.on_message(server, message.clone());
    }
}

struct Plugin {
    lib : Library,
}

impl Plugin {
    fn on_message(&mut self, server: &IrcServer, message: Message) -> () {
        unsafe {
            let on_message: Symbol<extern fn(server: &IrcServer, message: Message) -> ()> =
                self.lib.get(b"on_message").unwrap();

            on_message(server, message);
        }
    }
}

// DynPlugin
fn load_plugin(lib_path : String) -> Plugin {
    Plugin {
        lib : Library::new(lib_path).unwrap(),
    }
}
