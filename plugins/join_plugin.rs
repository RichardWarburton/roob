
extern crate modules;
extern crate irc;

use irc::client::prelude::*;
use irc::client::prelude::Command::*;
use std::any::Any;

use modules::*;

#[no_mangle]
pub fn on_message(server: &IrcServer, message: Message) {
    match message.command {
        PRIVMSG(_, text) => {
            match parse_command(text.clone()) {
                Args(cmd, channel) => {
                    if cmd == "join" {
                        server.send_join(&channel).unwrap();
                    }
                }
                _ => ()
            }
        }
        _ => (),
    }
}
