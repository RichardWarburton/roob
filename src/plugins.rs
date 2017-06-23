use irc::client::prelude::*;

const COMMAND_PREFIX : &str = "!";

pub trait Plugin {
    fn on_message(&mut self, server : &IrcServer, message : Message);
}

#[derive(Debug, PartialEq)]
pub enum Command {
    Cmd(String),
    Args(String, String),
    Other
}
pub use self::Command::*;

pub fn parse_command(text : String) -> Command {
    if text.starts_with(COMMAND_PREFIX) {
        match text.find(' ') {
            Some(index) => Args(
                String::from(&text[COMMAND_PREFIX.len() .. index]),
                String::from(&text[index + 1 .. ])),
            None => Cmd(String::from(&text[COMMAND_PREFIX.len() ..])),
        }
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
        assert_eq!(
            Args(String::from("hello"), String::from("world")),
            parse_command(String::from("!hello world")));
    }
}
