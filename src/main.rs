use crossterm::{
    cursor::{Hide, MoveTo, Show},
    event::{read, Event, KeyCode},
    execute,
    style::{Color, Print, ResetColor, SetForegroundColor},
    terminal::{self, Clear, ClearType, EnterAlternateScreen, LeaveAlternateScreen},
};
use rustyline::{error::ReadlineError, DefaultEditor, Result};
use std::{
    io::{stdout, Write},
    path::{Path, PathBuf},
};

fn get_history_file() -> Option<PathBuf> {
    let home_dir = std::env::var_os("HOME").or_else(|| std::env::var_os("USERPROFILE"))?;
    let config_dir_name = if cfg!(windows) {
        "AppData\\Roaming"
    } else {
        ".config"
    };
    let config_dir_path = Path::new(config_dir_name);
    Some(
        PathBuf::from(home_dir)
            .join(config_dir_path)
            .join("memtech_hist"),
    )
}

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

fn main() -> Result<()> {
    println!("Welcome to mt");
    let mut rl = DefaultEditor::new()?;
    if rl.load_history(&get_history_file().unwrap()).is_err() {
        println!("No previous history.");
        println!("Enter a text you want to memorize\n")
    }
    let out = stdout();
    loop {
        let _ = Life { write: &out };

        let readline = rl.readline("➜ ");
        match readline {
            Ok(text) => {
                if text.to_uppercase() == "EXIT" {
                    break;
                }
                rl.add_history_entry(text.as_str())?;
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
                    let mut s = false;
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
    Ok(())
}
