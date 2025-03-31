use std::io::{ self, Write };
use crossterm::{
    style::{ Color, Print, ResetColor, SetForegroundColor },
    terminal,
    ExecutableCommand,
};
use crossterm_input::{ input, InputEvent, KeyEvent };

fn main() -> std::io::Result<()> {
    let mut quit = false;
    let mut todos = vec!["Buy a bread", "Go to the market", "Make a cup of tea"];

    let input = input();
    let mut sync_stdin = input.read_async();

    let mut stdout = io::stdout();
    stdout.execute(terminal::Clear(terminal::ClearType::All))?;
    // or using functions
    stdout
        .execute(SetForegroundColor(Color::Blue))?
        .execute(Print("Styled text here."))?
        .execute(ResetColor)?;

    println!("");
    while !quit {
        if let Some(event) = sync_stdin.next() {
            match event {
                InputEvent::Keyboard(KeyEvent::Esc) => {
                    quit = true;
                }
                _ => {}
            }
        }
    }

    stdout.flush()?;
    Ok(())
}
