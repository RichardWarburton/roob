use irc::client::prelude::*;
use irc::client::prelude::Command::*;

use parse_command;
use Plugin;

pub struct JoinPlugin {}

impl Plugin for JoinPlugin {
    fn on_message(&mut self, server: &IrcServer, message: Message) {
        match message.command {
            PRIVMSG(_, text) => {
                match parse_command(text.clone()) {
                    Some(cmd) => {
                        if cmd == "join" {
                            let channel = text[6..].trim();
                            server.send_join(channel).unwrap();
                        }
                    }
                    None => ()
                }
            }
            _ => (),
        }
    }
}
