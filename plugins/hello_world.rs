
extern crate modules;
extern crate irc;

use irc::client::prelude::*;
use irc::client::prelude::Command::*;

//use modules::*;

#[no_mangle]
pub fn on_message(server: &IrcServer, message: Message) {
    match message.command {
        PRIVMSG(channel, text) => {
            if text == "Hi" {
                server.send(Message{
                    tags: None,
                    prefix: Some(String::from(server.current_nickname())),
                    command: PRIVMSG(channel, String::from("lo!"))
                }).expect("Couldn't send message");
            }
        },
        _ => (),
    }
}
