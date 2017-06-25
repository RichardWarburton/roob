use irc::client::prelude::*;
use irc::client::prelude::Command::*;

use modules::*;

pub struct JoinPlugin {}

impl Plugin for JoinPlugin {
    fn on_message(&mut self, server: &IrcServer, message: Message) {
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
}
