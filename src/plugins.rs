use irc::client::prelude::*;

const COMMAND_PREFIX : &str = "!";

pub trait Plugin {
    fn on_message(&mut self, server : &IrcServer, message : Message);
}

pub fn parse_command(text : String) -> Option<String> {
    if text.starts_with(COMMAND_PREFIX) {
        let end = text.find(' ').unwrap_or(text.len());
        Some(String::from(&text[COMMAND_PREFIX.len() .. end ]))
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_command_ignores_non_commands() {
        assert_eq!(
        None,
        parse_command(String::from("lalal")));
    }

    #[test]
    fn parse_command_parses_simple_command() {
        assert_eq!(
        Some(String::from("hi")),
        parse_command(String::from("!hi")));
    }

    #[test]
    fn parse_command_with_args() {
    }
}
