use std::io::stdout;
use std::time::Duration;
use std::thread::sleep;
use crossterm::{
    execute,
    style::{Color, Print, ResetColor, SetBackgroundColor, SetForegroundColor},
};

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
        Print("▄▀█  █▀█ █▀█ █▀█ ░░█ █▀▀ █▀▀ ▀█▀  █▀▀ █▀█ █▀█  █░█ █ █▀ ▀█▀ █▀█ █▀█ █▄█  ▀█▀ █▀█  █▀█ █▀█ █▀▀ █▀ █▀▀ █▄░█ ▀█▀\r\n█▀█  █▀▀ █▀▄ █▄█ █▄█ ██▄ █▄▄ ░█░  █▀░ █▄█ █▀▄  █▀█ █ ▄█ ░█░ █▄█ █▀▄ ░█░  ░█░ █▄█  █▀▀ █▀▄ ██▄ ▄█ ██▄ █░▀█ ░█░"),
        ResetColor).ok();
}

pub fn second() {
    clearscreen::clear().expect("err");
    execute!(
        stdout(),
        SetForegroundColor(Color::White),
        SetBackgroundColor(Color::Black),
        Print("
    - Billy the Kid was born as Henry McCarty in NYC in 1859\r
    - Born to Catherine and Patrick McCarty soon having a younger brother named Joseph\r
    - His father suddenly passed away and Catherine took herself and her two sons to Indianapolis\r
    - During this time Catherine began a romance with a William Antrim\r
    - Soon after the four moved to Wichita, Kansas, in 1870\r
    - They then moved to Santa Fe, where Henry's mom would marry Antrim in 1873, and later that same year they moved to Silver City, New Mexico\r
    - A year later Henry's mom died of tuberculosis leaving Henry and Joseph in the care of their step-father\r
    - William would beat Henry and with the death of Henry's mother they became even worse as she wasn't there to protect her child.\r
    - Henry left home and began to live in a boarding house in which he would do work in exchange for living quarters.\r"),
        ResetColor).ok();
}

pub fn third() {
    clearscreen::clear().expect("err");
    execute!(
        stdout(),
        SetForegroundColor(Color::White),
        SetBackgroundColor(Color::Black),
        Print("deez"),
        ResetColor).ok();
}
pub fn fourth() {
    clearscreen::clear().expect("err");
    execute!(
        stdout(),
        SetForegroundColor(Color::White),
        SetBackgroundColor(Color::Black),
        Print("nuts"),
        ResetColor).ok();
}
pub fn fifth() {
    clearscreen::clear().expect("err");
    execute!(
        stdout(),
        SetForegroundColor(Color::White),
        SetBackgroundColor(Color::Black),
        Print("hey"),
        ResetColor).ok();
}
pub fn sixth() {
    clearscreen::clear().expect("err");
    execute!(
        stdout(),
        SetForegroundColor(Color::White),
        SetBackgroundColor(Color::Black),
        Print("gottem"),
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