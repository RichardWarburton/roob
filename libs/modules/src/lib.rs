extern crate irc;

use irc::client::prelude::*;

const COMMAND_PREFIX : &str = "!";

pub trait Plugin {
    fn on_message(&mut self, server : &IrcServer, message : Message);
}

#[macro_export]
macro_rules! plugin {
    () => {
        static mut state : *mut State = 0 as *mut State;

        #[no_mangle]
        pub fn plugin_init(server: &IrcServer) -> () {
            let state_val = init(server);
            unsafe { state = mem::transmute(Box::new(state_val)); }
        }

        #[no_mangle]
        pub fn plugin_on_message(server: &IrcServer, message: Message) {
            let ref mut context = unsafe {&mut *state};
            on_message(context, server, message);
        }
    };
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
