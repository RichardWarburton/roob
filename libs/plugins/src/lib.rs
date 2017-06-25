
extern crate modules;
extern crate irc;

use irc::client::prelude::*;
use irc::client::prelude::Command::*;

use modules::*;

struct HelloWorldPlugin {

}

impl Plugin for HelloWorldPlugin {
    fn on_message(&mut self, server: &IrcServer, message: Message) {
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
}

#[no_mangle]
pub fn get_plugin() -> Box<Plugin> {
    Box::new(HelloWorldPlugin {})
}

#[no_mangle]
pub fn get_val() -> Box<u32> {
    Box::new(4)
}
