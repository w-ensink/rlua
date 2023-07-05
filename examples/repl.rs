//! This example shows a simple read-evaluate-print-loop (REPL).

use rlua::{Error, Lua, MultiValue};
use rustyline::DefaultEditor;

fn main() {
    Lua::new().context(|lua| {
        // let mut editor = Editor::<()>::new();
        let rl_c = rustyline::DefaultEditor::new();
        match rl_c {
            Ok(mut editor) => {
                loop {
                    let mut prompt = "> ";
                    let mut line = String::new();
                    loop {
                        match editor.readline(prompt) {
                            Ok(input) => line.push_str(&input),
                            Err(_) => return,
                        }
                        match lua.load(&line).eval::<MultiValue>() {
                            Ok(values) => {
                                editor.add_history_entry(line);
                                println!(
                                    "{}",
                                    values
                                        .iter()
                                        .map(|value| format!("{:?}", value))
                                        .collect::<Vec<_>>()
                                        .join("\t")
                                );
                                break;
                            }
                            Err(Error::SyntaxError {
                                incomplete_input: true,
                                ..
                            }) => {
                                // continue reading input and append it to `line`
                                line.push_str("\n"); // separate input lines
                                prompt = ">> ";
                            }
                            Err(e) => {
                                eprintln!("error: {}", e);
                                break;
                            }
                        }
                    }
                }
            }
            Err(e) => eprintln!("error: {}", e),
        }
    });
}
