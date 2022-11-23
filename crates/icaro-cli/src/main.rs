use rustyline::hint::{Hint, Hinter};
use rustyline::Context;
use rustyline::{Editor, Result};
use rustyline_derive::{Completer, Helper, Highlighter, Validator};
use yansi::Paint;

#[derive(Completer, Helper, Validator, Highlighter)]
struct LispHinter {}

#[derive(Hash, Debug, PartialEq, Eq)]
struct CommandHint {
    word: String,
    colored: String,
}

impl Hint for CommandHint {
    fn display(&self) -> &str {
        &self.colored
    }

    fn completion(&self) -> Option<&str> {
        Some(&self.word)
    }
}

impl CommandHint {
    fn new(text: &str) -> CommandHint {
        CommandHint {
            word: text.into(),
            colored: Paint::new(text).dimmed().to_string(),
        }
    }

    fn suffix(&self, strip_chars: usize) -> CommandHint {
        CommandHint {
            word: self.word[strip_chars..].to_owned(),
            colored: Paint::new(self.word[strip_chars..].to_owned())
                .dimmed()
                .to_string(),
        }
    }
}

impl Hinter for LispHinter {
    type Hint = CommandHint;

    fn hint(&self, line: &str, pos: usize, _ctx: &Context<'_>) -> Option<CommandHint> {
        if line.is_empty() || pos < line.len() {
            return None;
        }

        if "pudding".starts_with(line) {
            Some(CommandHint::new("pudding").suffix(line.len()))
        } else {
            None
        }
    }
}

fn main() -> Result<()> {
    let h = LispHinter {};

    let mut rl: Editor<LispHinter> = Editor::new()?;
    rl.set_helper(Some(h));

    loop {
        let input = rl.readline("> ")?;
        println!("{input}");
    }
}
