use irc::client::prelude::*;
use irc::client::prelude::Command::*;

use plugins::*;

pub struct JoinPlugin {}

impl Plugin for JoinPlugin {
    fn on_message(&mut self, server: &IrcServer, message: Message) {
        match message.command {
            PRIVMSG(_, text) => {
                match parse_command(text.clone()) {
                    Cmd(cmd) => {
                        if cmd == "join" {
                            let channel = text[6..].trim();
                            server.send_join(channel).unwrap();
                        }
                    }
                    Other => ()
                }
            }
            _ => (),
        }
    }
}
