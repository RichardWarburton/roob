
extern crate irc;
extern crate regex;
extern crate modules;

use irc::client::prelude::*;

mod join_plugin;
mod karma_plugin;

use modules::*;

extern crate libloading as lib;

use lib::{Library, Symbol};

// DynPlugin approach:
/*struct DynPlugin {
    lib : Library,
}

impl DynPlugin {
    fn on_message(self, server: &IrcServer, message: Message) -> () {
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
}*/

fn load_plugins() -> lib::Result<Box<Plugin>> {
    let lib = try!(Library::new("libs/plugins/target/debug/plugins.dll"));

    unsafe {
        let get_plugin: Symbol<extern "Rust" fn() -> Box<Plugin>> =
            try!(lib.get(b"get_plugin"));

        Ok(get_plugin())
    }
}

fn sigh() -> lib::Result<Box<u32>> {
    let lib = try!(Library::new("libs/plugins/target/debug/plugins.dll"));

    unsafe {
        let get_plugin: Symbol<extern fn() -> Box<u32>> =
            try!(lib.get(b"get_val"));

        Ok(get_plugin())
    }
}

fn main() {
    match sigh() {
        Ok(val) => println!("Val: {}", val),
        Err(e) => println!("Error: {:?}", e),
    }

    match load_plugins() {
        Ok(plugin) => {
            let server = IrcServer::new("config.json").unwrap();
            let mut plugins : Vec<Box<Plugin>> = Vec::new();
            plugins.push(plugin);
            plugins.push(Box::new(karma_plugin::KarmaPlugin::new()));
            plugins.push(Box::new(join_plugin::JoinPlugin{}));
            server.identify().unwrap();
            for message in server.iter() {
                match message {
                    Err(e) => println!("Error: {:?}", e),
                    Ok(msg) => handle_message(&mut plugins, &server, msg),
                }
            }
        },
        Err(e) => println!("Error: {:?}", e),
    }
}

fn handle_message(plugins : &mut Vec<Box<Plugin>>, server : &IrcServer, message : Message) {
    // TODO: better logging
    println!("{:?}", message);

    for plugin in plugins {
        plugin.on_message(server, message.clone());
    }
}
