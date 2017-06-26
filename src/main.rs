
extern crate irc;
extern crate regex;
extern crate modules;

use std::fs;
use std::process;
use std::io::Write;
use std::path::Path;
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
fn load_plugin(lib_path : String) -> DynPlugin {
    DynPlugin {
        lib : Library::new(lib_path).unwrap(),
    }
}

fn main() {

    let plugin_files = fs::read_dir("plugins").unwrap();

    let plugin_target = Path::new("plugin_bin");

    if !plugin_target.exists() {
        fs::create_dir(plugin_target).unwrap();
    }

    let mut plugin_lib_paths : Vec<String> = Vec::new();

    for plugin_file in plugin_files {
        let file = plugin_file.unwrap();
        let os_file_name = file.file_name();
        let file_name = os_file_name.to_str().unwrap();
        if file_name.ends_with(".rs") {
            let plugin_name = String::from(&file_name[..file_name.len() - 3]);
            println!("Name: {} {}", file.path().display(), plugin_name);

            // create dir
            let plugin_dir = plugin_target.to_str().unwrap().to_owned() + "/" + &plugin_name + "/";
            let plugin_path = Path::new(&plugin_dir);
            let src_dir = plugin_dir.clone() + "/src";

            // TODO: would it be better to just use rustc?
            // TODO: have a way to update the cargo file
            if (!plugin_path.exists())
            {
                fs::create_dir(plugin_path).unwrap();

                // write cargo file
                let cargo_path = plugin_dir.clone() + "Cargo.toml";
                let mut cargo_file = fs::File::create(cargo_path).unwrap();
                write!(cargo_file, "
                [package]
                name = \"{}\"
                version = \"0.1.0\"
                authors = [\"Richard Warburton <richard.warburton@gmail.com>\"]

                [lib]
                crate-type = [\"dylib\"]

                [dependencies]
                irc = {{ git = \"https://github.com/RichardWarburton/irc.git\" }}

                [dependencies.modules]
                path = \"../../libs/modules\"
                ", plugin_name).unwrap();
                cargo_file.flush().unwrap();

                //
                fs::create_dir(&src_dir).unwrap();
            }

            // TODO: don't update if the file wasn't changed.

            fs::copy(file.path(), src_dir.to_owned() + "/lib.rs").unwrap();

            // Cargo build
            let output = process::Command::new("cargo")
                .current_dir(plugin_path)
                .args(&["build"])
                .output()
                .expect("failed to execute process");

            if !output.status.success() {
                println!("{}", String::from_utf8_lossy(&output.stdout));
                println!("{}", String::from_utf8_lossy(&output.stderr));
            } else {
                let lib_path = plugin_dir.clone() + "target/debug/" + &plugin_name + ".dll";
                plugin_lib_paths.push(lib_path);
            }
        }
    }

    start_bot(plugin_lib_paths);
}

fn start_bot(plugin_lib_paths : Vec<String>) {
    let mut plugins: Vec<DynPlugin> = Vec::new();
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

fn handle_message(plugins : &mut Vec<DynPlugin>, server : &IrcServer, message : Message) {
    // TODO: better logging
    println!("{:?}", message);

    for plugin in plugins {
        plugin.on_message(server, message.clone());
    }
}
