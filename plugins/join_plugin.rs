
extern crate modules;
extern crate irc;

use irc::client::prelude::*;
use irc::client::prelude::Command::*;
use std::any::Any;

use modules::*;

pub struct State {
}

#[no_mangle]
pub fn init() -> Box<State> {
    Box::new(State{})
}

#[no_mangle]
pub fn on_message(state: &mut Any, server: &IrcServer, message: Message) {
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
