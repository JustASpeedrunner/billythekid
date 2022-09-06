use std::io::stdout;
use std::time::Duration;
use std::thread::sleep;
use crossterm::{
    execute,
    style::{Color, Print, ResetColor, SetBackgroundColor, SetForegroundColor},
};
use crossterm::terminal::Clear;

pub fn autocheck(sldnum:i32) {
    match sldnum {
        0 => mainslide(),
        1 => second(),
        2 => third(),
        3 => fourth(),
        4 => fifth(),
        5 => sixth(),
        6 => sources(),
        2147483647 => goodbye(),

        _ => err(),
    }
}

pub fn mainslide() {
    clearscreen::clear().expect("err");
    execute!(
        stdout(),
        SetForegroundColor(Color::White),
        SetBackgroundColor(Color::Black),
        Print("▄▀█  █▀█ █▀█ █▀█ ░░█ █▀▀ █▀▀ ▀█▀  █▀▀ █▀█ █▀█  █░█ █ █▀ ▀█▀ █▀█ █▀█ █▄█  ▀█▀ █▀█  █▀█ █▀█ █▀▀ █▀ █▀▀ █▄░█ ▀█▀\n█▀█  █▀▀ █▀▄ █▄█ █▄█ ██▄ █▄▄ ░█░  █▀░ █▄█ █▀▄  █▀█ █ ▄█ ░█░ █▄█ █▀▄ ░█░  ░█░ █▄█  █▀▀ █▀▄ ██▄ ▄█ ██▄ █░▀█ ░█░"),
        ResetColor).ok();
}

pub fn second() {
    Clear(crossterm::terminal::ClearType::All);
    execute!(
        stdout(),
        SetForegroundColor(Color::White),
        SetBackgroundColor(Color::Black),
        Print(""),
        ResetColor).ok();
}

pub fn third() {
    Clear(crossterm::terminal::ClearType::All);
    execute!(
        stdout(),
        SetForegroundColor(Color::White),
        SetBackgroundColor(Color::Black),
        Print(""),
        ResetColor).ok();
}
pub fn fourth() {
    Clear(crossterm::terminal::ClearType::All);
    execute!(
        stdout(),
        SetForegroundColor(Color::White),
        SetBackgroundColor(Color::Black),
        Print(""),
        ResetColor).ok();
}
pub fn fifth() {
    Clear(crossterm::terminal::ClearType::All);
    execute!(
        stdout(),
        SetForegroundColor(Color::White),
        SetBackgroundColor(Color::Black),
        Print(""),
        ResetColor).ok();
}
pub fn sixth() {
    Clear(crossterm::terminal::ClearType::All);
    execute!(
        stdout(),
        SetForegroundColor(Color::White),
        SetBackgroundColor(Color::Black),
        Print(""),
        ResetColor).ok();
}


pub fn goodbye() {
    clearscreen::clear().expect("err");
    execute!(
        stdout(),
        SetForegroundColor(Color::White),
        SetBackgroundColor(Color::Black),
        Print("Thank you for listening to my presentation."),
        ResetColor
    ).ok();
    sleep(Duration::from_secs(2));
    clearscreen::clear().expect("err");
}

pub fn sources() {
    clearscreen::clear().expect("err");
    execute!(
        stdout(),
        SetForegroundColor(Color::White),
        SetBackgroundColor(Color::Black),
        Print("# Sources\r\n\"Absolute Mad Lads - Billy the Kid\" YouTube, uploaded by Count Dankula, 13-12-2019, https://www.youtube.com/watch?v=QjWqfV3j-eY\"\r\nGarrett, Pat F. (1882). The Authentic Life of Billy, the Kid (1st ed.). Santa Fe, New Mexico: New Mexican Printing and Publishing Company. OCLC 748293298.\r\n[Billy the Kid photograph taken by Ben Wittick in 1879 or 1880.](https://commons.wikimedia.org/wiki/File:Billy_the_Kid_tintype,_Fort_Sumner,_1879-80-Edit2.jpg)"),
        ResetColor).ok();
}

pub fn err() {
    println!("This is not a valid value, please try again.");
}