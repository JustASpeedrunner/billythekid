
// -----------------------------------------------------------------
// Import sh-, oh wait this is a school project, import stuff. Yeah, stuff.
// -----------------------------------------------------------------

use std::io::{stdout, Write};
use std::time::Duration;
use std::thread::sleep;
use crossterm::{
    execute, terminal::{Clear},
    style::{Color, Print, ResetColor, SetBackgroundColor, SetForegroundColor},
    ExecutableCommand, Result,
    event,
};
use crossterm::event::{
    poll, KeyboardEnhancementFlags, PopKeyboardEnhancementFlags, PushKeyboardEnhancementFlags, read, DisableBracketedPaste, DisableFocusChange, DisableMouseCapture, EnableBracketedPaste, EnableFocusChange, EnableMouseCapture, Event, KeyCode,
};
use crossterm::{
    execute,
    terminal::{disable_raw_mode, enable_raw_mode},
    Result,
};

use termimad;

mod slides;

// -----------------------------------------------------------------
// Let the code begin
// -----------------------------------------------------------------

fn main() -> Result<()> {
    println!("{}", slides::mainslides);

    enable_raw_mode()?;

    let mut stdout = stdout();
    execute!(
        stdout,
        EnableBracketedPaste,
        EnableFocusChange,
        EnableMouseCapture,
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
        DisableMouseCapture
    )?;

    disable_raw_mode()
}


fn read_events() -> Result<()> {
    loop {
        let event = read()?;
        let pub mut curslidecnt=1;

        println!("Event: {:?}\r", event);

        match event {
            Event::Key(KeyCode::Enter.into()) => 
                Clear;
                match curslidecnt {
                    1..=5 => curslidecnt = curslidecnt++; slides::autocheck();
                    cur if cur > 6 => slides::mainslide();    
                },
            Event::Key(KeyCode::Char('s').into()) => slides::sources(),
            Event::Key(KeyCode::Esc.into()) => 
                curslidecnt = 2147483647;
                slides::autocheck();
        }
    }

    Ok(())
}
