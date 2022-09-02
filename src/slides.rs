pub fn autocheck(&mut sldnum:i32) {
    match sldnum {
        0 => mainslide(),
        1 => second(),
        2 => third(),
        3 => fourth(),
        4 => fifth(),
        5 => sixth(),
        6 => sources(),

        2147483647 => goodbye(),
    }
}

pub fn mainslide() {
    execute!(
        stdout(),
        SetForegroundColor(Color::White),
        SetBackgroundColor(Color::Black),
        Print("▄▀█  █▀█ █▀█ █▀█ ░░█ █▀▀ █▀▀ ▀█▀  █▀▀ █▀█ █▀█  █░█ █ █▀ ▀█▀ █▀█ █▀█ █▄█  ▀█▀ █▀█  █▀█ █▀█ █▀▀ █▀ █▀▀ █▄░█ ▀█▀
               █▀█  █▀▀ █▀▄ █▄█ █▄█ ██▄ █▄▄ ░█░  █▀░ █▄█ █▀▄  █▀█ █ ▄█ ░█░ █▄█ █▀▄ ░█░  ░█░ █▄█  █▀▀ █▀▄ ██▄ ▄█ ██▄ █░▀█ ░█░")
        Print("▄▀█ █▄░█ █▀▄  ▀█▀ █▀▀ ▄▀█ █▀▀ █░█  ▄▀█ █▄▄ █▀█ █░█ ▀█▀  ▀█▀ █░█ █▀▀  █▀ ▀█▀ █▀█ █▀█ █▄█  █▀█ █▀▀ ▀
               █▀█ █░▀█ █▄▀  ░█░ ██▄ █▀█ █▄▄ █▀█  █▀█ █▄█ █▄█ █▄█ ░█░  ░█░ █▀█ ██▄  ▄█ ░█░ █▄█ █▀▄ ░█░  █▄█ █▀░ ▄"),
        sleep(Duration::from_secs(2)),
        Print("██████╗░██╗██╗░░░░░██╗░░░░░██╗░░░██╗ ████████╗██╗░░██╗███████╗ ██╗░░██╗██╗██████╗░
               ██╔══██╗██║██║░░░░░██║░░░░░╚██╗░██╔╝ ╚══██╔══╝██║░░██║██╔════╝ ██║░██╔╝██║██╔══██╗
               ██████╦╝██║██║░░░░░██║░░░░░░╚████╔╝░ ░░░██║░░░███████║█████╗░░ █████═╝░██║██║░░██║
               ██╔══██╗██║██║░░░░░██║░░░░░░░╚██╔╝░░ ░░░██║░░░██╔══██║██╔══╝░░ ██╔═██╗░██║██║░░██║
               ██████╦╝██║███████╗███████╗░░░██║░░░ ░░░██║░░░██║░░██║███████╗ ██║░╚██╗██║██████╔╝
               ╚═════╝░╚═╝╚══════╝╚══════╝░░░╚═╝░░░ ░░░╚═╝░░░╚═╝░░╚═╝╚══════╝ ╚═╝░░╚═╝╚═╝╚═════╝░"),        
        Print(" - Hit Enter to continue to the next slide.
                - Using the number keys above your keyboard you can select slides to go to.
                  - 0 is this slide and _ is the last slide.
                - For the sources hit the 's' key.
                - Use Esc to quit"),
        ResetColor
    )?;
}

pub fn second() {
    execute!(
        stdout(),
        SetForegroundColor(Color::White),
        SetBackgroundColor(Color::Black),
        Print(""),
        ResetColor)?;
}

pub fn third() {
    execute!(
        stdout(),
        SetForegroundColor(Color::White),
        SetBackgroundColor(Color::Black),
        Print(""),
        ResetColor)?;
}
pub fn fourth() {
    execute!(
        stdout(),
        SetForegroundColor(Color::White),
        SetBackgroundColor(Color::Black),
        Print(""),
        ResetColor)?;
}
pub fn fifth() {
    execute!(
        stdout(),
        SetForegroundColor(Color::White),
        SetBackgroundColor(Color::Black),
        Print(""),
        ResetColor)?;
}
pub fn sixth() {
    execute!(
        stdout(),
        SetForegroundColor(Color::White),
        SetBackgroundColor(Color::Black),
        Print(""),
        ResetColor)?;
}


pub fn goodbye() {
    execute!(
        stdout(),
        SetForegroundColor(Color::White),
        SetBackgroundColor(Color::Black),
        Print("Thank you for listening to my presentation, and thank you to Mr. Lonsway for being the best teacher ever."),
        ResetColor
    )?;
    sleep(Duration::from_secs(5));
}

pub fn sources() {
    let mut skin = MadSkin::default();
    skin.header.add_attr(Underlined);
    println!("# Sources\n\"Absolute Mad Lads - Billy the Kid\" YouTube, uploaded by Count Dankula, 13-12-2019, https://www.youtube.com/watch?v=QjWqfV3j-eY\nGarrett, Pat F. (1882). The Authentic Life of Billy, the Kid (1st ed.). Santa Fe, New Mexico: New Mexican Printing and Publishing Company. OCLC 748293298.\n[Billy the Kid photograph taken by Ben Wittick in 1879 or 1880.](https://commons.wikimedia.org/wiki/File:Billy_the_Kid_tintype,_Fort_Sumner,_1879-80-Edit2.jpg)");
}

pub fn err() {
    println!("This is not a valid value, please try again.");
}