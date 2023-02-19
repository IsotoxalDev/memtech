use crossterm::{
    cursor::{Hide, MoveTo, Show},
    event::{read, Event, KeyCode},
    execute,
    style::{Color, Print, ResetColor, SetForegroundColor},
    terminal::{self, Clear, ClearType, EnterAlternateScreen, LeaveAlternateScreen},
};
use rustyline::error::ReadlineError;
use rustyline::Editor;
use std::io::{stdin, stdout, Write};

struct Life<W: Write> {
    write: W,
}
impl<W> Drop for Life<W>
where
    W: Write,
{
    fn drop(&mut self) {
        execute!(self.write, Show, LeaveAlternateScreen).unwrap();
        terminal::disable_raw_mode().unwrap();
    }
}

fn main() {
    println!("Welcome to mt");
    let mut rl = Editor::<()>::new().unwrap();
    if rl.load_history("mt_hist").is_err() {
        println!("No previous history.");
    }
    let out = stdout();
    loop {
        let _ = Life { write: &out };
        let mut s = false;

        let readline = rl.readline("➜ ");
        match readline {
            Ok(text) => {
                rl.add_history_entry(text.as_str());
                if text.to_uppercase() == "EXIT" {
                    break;
                }
                execute!(
                    &out,
                    Hide,
                    EnterAlternateScreen,
                    MoveTo(1, 1),
                    SetForegroundColor(Color::Blue),
                    Print(&text),
                    ResetColor,
                )
                .unwrap();
                terminal::enable_raw_mode().unwrap();
                loop {
                    if let Event::Key(event) = read().unwrap() {
                        match event.code {
                            KeyCode::Esc => break,
                            KeyCode::Enter => {
                                s = !s;
                                if s {
                                    let fls: String = text
                                        .clone()
                                        .split_whitespace()
                                        .map(|word| {
                                            format!("{} ", word.chars().nth(0).unwrap().to_string())
                                        })
                                        .collect();
                                    execute!(
                                        &out,
                                        Clear(ClearType::All),
                                        MoveTo(1, 1),
                                        SetForegroundColor(Color::Blue),
                                        Print(fls),
                                        ResetColor,
                                    )
                                    .unwrap();
                                    continue;
                                }
                                execute!(
                                    &out,
                                    Hide,
                                    EnterAlternateScreen,
                                    MoveTo(1, 1),
                                    SetForegroundColor(Color::Blue),
                                    Print(&text),
                                    ResetColor,
                                )
                                .unwrap()
                            }
                            _ => continue,
                        }
                    }
                }
            }
            Err(ReadlineError::Interrupted) => {
                println!("CTRL-C");
                break;
            }
            Err(ReadlineError::Eof) => {
                println!("CTRL-D");
                break;
            }
            Err(err) => {
                println!("Error: {:?}", err);
                break;
            }
        }
        rl.save_history("mt_hist").unwrap();
    }
}
