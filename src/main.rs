
extern crate irc;
extern crate regex;
extern crate modules;
extern crate libloading as lib;

mod karma_plugin;
mod horrible_perl_script;

use irc::client::prelude::*;
use lib::{Library, Symbol};
use horrible_perl_script::setup_plugins;

use std::any::Any;

fn main() {
    let plugin_lib_paths = setup_plugins();
    start_bot(plugin_lib_paths);
}

fn start_bot(plugin_lib_paths : Vec<String>) {
    let server = IrcServer::new("config.json").unwrap();
    server.identify().unwrap();

    let mut plugins: Vec<Plugin> = Vec::new();
    for lib_path in plugin_lib_paths {
        plugins.push(load_plugin(lib_path, &server));
    }

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
    state : Box<Any>,
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

fn load_plugin(lib_path : String, server: &IrcServer) -> Plugin {
    let lib = Library::new(lib_path).unwrap();
    let state = unsafe {
        let init_state : Result<Symbol<extern fn() -> Box<Any>>, _> =
            lib.get(b"init_state");

        match init_state {
            Ok(func) => func(),
            Err(_) => Box::new(""),
        }
    };

    Plugin {
        state : state,
        lib : lib,
    }
}
