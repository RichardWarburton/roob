use irc::client::prelude::*;
use irc::client::prelude::Command::*;

use Plugin;

pub struct HelloWorldPlugin {

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
