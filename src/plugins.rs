use irc::client::prelude::*;

const COMMAND_PREFIX : &str = "!";

pub trait Plugin {
    fn on_message(&mut self, server : &IrcServer, message : Message);
}

#[derive(Debug, PartialEq)]
pub enum Command {
    Cmd(String),
    Other
}
pub use self::Command::*;

pub fn parse_command(text : String) -> Command {
    if text.starts_with(COMMAND_PREFIX) {
        let end = text.find(' ').unwrap_or(text.len());
        Cmd(String::from(&text[COMMAND_PREFIX.len() .. end ]))
    } else {
        Other
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_command_ignores_non_commands() {
        assert_eq!(
        Other,
        parse_command(String::from("lalal")));
    }

    #[test]
    fn parse_command_parses_simple_command() {
        assert_eq!(
        Cmd(String::from("hi")),
        parse_command(String::from("!hi")));
    }

    #[test]
    fn parse_command_with_args() {
    }
}
