// -----------------------------------------------------------------
// Just a line to build for Michaelsoft Binbows so I don't forget
// cargo build --target x86_64-pc-windows-gnu --release
// -----------------------------------------------------------------

// -----------------------------------------------------------------
// Import sh-, oh wait this is a school project, import stuff. Yeah, stuff.
// -----------------------------------------------------------------

use std::io::{stdout};
use crossterm::terminal::{disable_raw_mode, enable_raw_mode, SetSize};
use crossterm::{
    execute, Result,
};
use crossterm::event::{
    read, DisableBracketedPaste, DisableFocusChange, EnableBracketedPaste, EnableFocusChange, Event, KeyCode,
};
mod slides;

// -----------------------------------------------------------------
// Let the code begin
// -----------------------------------------------------------------

fn main() -> Result<()> {
    SetSize(109,100);

    slides::mainslide();

    enable_raw_mode()?;

    let mut stdout = stdout();
    execute!(
        stdout,
        EnableBracketedPaste,
        EnableFocusChange,
    )?;

    if let Err(e) = read_events() {
        println!("Error: {:?}\r", e);
    }

    execute!(
        stdout,
        DisableBracketedPaste,
        DisableFocusChange,
    )?;

    disable_raw_mode()
}


fn read_events() -> Result<()> {
    let mut curslidecnt = 0;
    loop {
        let event = read()?;
        println!("{:?}",event);

        if event == Event::Key(KeyCode::Enter.into()) {
            if let 0..=8 = curslidecnt {
                curslidecnt += 1;
                slides::autocheck(curslidecnt);
            } else if curslidecnt >= 9 {
                curslidecnt = 0;
                clearscreen::clear().expect("err");
                slides::autocheck(curslidecnt);
            };
        };
        if event == Event::Key(KeyCode::Esc.into()) {
            curslidecnt = 2147483647;
            slides::autocheck(curslidecnt);
            break;
        };
        if event == Event::Key(KeyCode::Char('s').into()) {
            curslidecnt = 6;
            slides::autocheck(curslidecnt);
        };
    };
Ok(())
}