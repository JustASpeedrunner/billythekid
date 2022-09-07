// -----------------------------------------------------------------
// Just a line to build for Michaelsoft Binbows so I don't forget
// cargo build --target x86_64-pc-windows-gnu --release
// -----------------------------------------------------------------

// -----------------------------------------------------------------
// Import sh-, oh wait this is a school project, import stuff. Yeah, stuff.
// -----------------------------------------------------------------

use std::io::{stdout};
use crossterm::terminal::{disable_raw_mode, enable_raw_mode};
use crossterm::{
    execute, Result,
};
use crossterm::event::{
    KeyboardEnhancementFlags, PopKeyboardEnhancementFlags, PushKeyboardEnhancementFlags, read, DisableBracketedPaste, DisableFocusChange, EnableBracketedPaste, EnableFocusChange, Event, KeyCode,
};
    mod slides;

// -----------------------------------------------------------------
// Let the code begin
// -----------------------------------------------------------------

fn main() -> Result<()> {
    slides::mainslide();

    enable_raw_mode()?;

    let mut stdout = stdout();
    execute!(
        stdout,
        EnableBracketedPaste,
        EnableFocusChange,
        PushKeyboardEnhancementFlags(
            KeyboardEnhancementFlags::DISAMBIGUATE_ESCAPE_CODES
                | KeyboardEnhancementFlags::REPORT_ALL_KEYS_AS_ESCAPE_CODES
                | KeyboardEnhancementFlags::REPORT_EVENT_TYPES
        )
    )?;

    if let Err(e) = read_events() {
        println!("Error: {:?}\r", e);
    }

    execute!(
        stdout,
        DisableBracketedPaste,
        PopKeyboardEnhancementFlags,
        DisableFocusChange,
    )?;

    disable_raw_mode()
}


fn read_events() -> Result<()> {
    loop {
        let event = read()?;
        let mut curslidecnt = 1;
        println!("{:?}",event);

        if event == Event::Key(KeyCode::Enter.into()) {
            if let 1..=5 = curslidecnt {
                curslidecnt += 1;
                slides::autocheck(curslidecnt);
            } else if curslidecnt > 6 {
                slides::mainslide();
            };
        }
        if event == Event::Key(KeyCode::Esc.into()) {
            curslidecnt = 2147483647;
            slides::autocheck(curslidecnt);
            break;
        }
        if event == Event::Key(KeyCode::Char('s').into()) {
            curslidecnt = 6;
            slides::autocheck(curslidecnt);
        }
    }
Ok(())
}