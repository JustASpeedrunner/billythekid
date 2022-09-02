use std::io::{stdout, Write};
use crossterm::{
    execute, terminal::{*, clear},
    style::{Color, Print, ResetColor, SetBackgroundColor, SetForegroundColor},
    ExecutableCommand, Result,
    event,
};

fn main() -> Result<()> {

    execute!(
        stdout(),
        SetForegroundColor(Color::White),
        SetBackgroundColor(Color::Black),
        Print("██████╗░██╗██╗░░░░░██╗░░░░░██╗░░░██╗ ████████╗██╗░░██╗███████╗ ██╗░░██╗██╗██████╗░
               ██╔══██╗██║██║░░░░░██║░░░░░╚██╗░██╔╝ ╚══██╔══╝██║░░██║██╔════╝ ██║░██╔╝██║██╔══██╗
               ██████╦╝██║██║░░░░░██║░░░░░░╚████╔╝░ ░░░██║░░░███████║█████╗░░ █████═╝░██║██║░░██║
               ██╔══██╗██║██║░░░░░██║░░░░░░░╚██╔╝░░ ░░░██║░░░██╔══██║██╔══╝░░ ██╔═██╗░██║██║░░██║
               ██████╦╝██║███████╗███████╗░░░██║░░░ ░░░██║░░░██║░░██║███████╗ ██║░╚██╗██║██████╔╝
               ╚═════╝░╚═╝╚══════╝╚══════╝░░░╚═╝░░░ ░░░╚═╝░░░╚═╝░░╚═╝╚══════╝ ╚═╝░░╚═╝╚═╝╚═════╝░"),
        ResetColor
    )?;

    Ok(())
}