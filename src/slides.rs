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
        Print("

        ▄▀█  █▀█ █▀█ █▀█ ░░█ █▀▀ █▀▀ ▀█▀  █▀▀ █▀█ █▀█  █░█ █ █▀ ▀█▀ █▀█ █▀█ █▄█  ▀█▀ █▀█  █▀█ █▀█ █▀▀ █▀ █▀▀ █▄░█ ▀█▀  ▄▀█ █▄░█ █▀▄  ▀█▀ █▀▀ ▄▀█ █▀▀ █░█  ▄▀█ █▄▄ █▀█ █░█ ▀█▀\r
        █▀█  █▀▀ █▀▄ █▄█ █▄█ ██▄ █▄▄ ░█░  █▀░ █▄█ █▀▄  █▀█ █ ▄█ ░█░ █▄█ █▀▄ ░█░  ░█░ █▄█  █▀▀ █▀▄ ██▄ ▄█ ██▄ █░▀█ ░█░  █▀█ █░▀█ █▄▀  ░█░ ██▄ █▀█ █▄▄ █▀█  █▀█ █▄█ █▄█ █▄█ ░█░\r

                                                                    ▀█▀ █░█ █▀▀  █▀ ▀█▀ █▀█ █▀█ █▄█  █▀█ █▀▀\r
                                                                    ░█░ █▀█ ██▄  ▄█ ░█░ █▄█ █▀▄ ░█░  █▄█ █▀░\r\n"),
        ResetColor
    ).ok();

    sleep(Duration::from_secs(2));
    
    execute!(
        stdout(),
        SetForegroundColor(Color::White),
        SetBackgroundColor(Color::Black),
        Print("
                                                ██████╗░██╗██╗░░░░░██╗░░░░░██╗░░░██╗  ████████╗██╗░░██╗███████╗  ██╗░░██╗██╗██████╗░\r
                                                ██╔══██╗██║██║░░░░░██║░░░░░╚██╗░██╔╝  ╚══██╔══╝██║░░██║██╔════╝  ██║░██╔╝██║██╔══██╗\r
                                                ██████╦╝██║██║░░░░░██║░░░░░░╚████╔╝░  ░░░██║░░░███████║█████╗░░  █████═╝░██║██║░░██║\r
                                                ██╔══██╗██║██║░░░░░██║░░░░░░░╚██╔╝░░  ░░░██║░░░██╔══██║██╔══╝░░  ██╔═██╗░██║██║░░██║\r
                                                ██████╦╝██║███████╗███████╗░░░██║░░░  ░░░██║░░░██║░░██║███████╗  ██║░╚██╗██║██████╔╝\r
                                                ╚═════╝░╚═╝╚══════╝╚══════╝░░░╚═╝░░░  ░░░╚═╝░░░╚═╝░░╚═╝╚══════╝  ╚═╝░░╚═╝╚═╝╚═════╝░\r"),
        ResetColor
    ).ok();
    sleep(Duration::from_secs(2));
    execute!(
        stdout(),
        SetForegroundColor(Color::White),
        SetBackgroundColor(Color::Black),
        Print("\n\n
                                                                - Hit Enter to continue to the next slide.\r\n
                                                                - For the sources hit the 's' key.\r\n
                                                                - Use Esc to quit\r"),
        ResetColor).ok();
}

pub fn second() {
    clearscreen::clear().expect("err");
    execute!(
        stdout(),
        SetForegroundColor(Color::White),
        SetBackgroundColor(Color::Black),
        Print("
░░███╗░░██╗░░  ███████╗░█████╗░██████╗░██╗░░░░░██╗░░░██╗  ██╗░░░░░██╗███████╗███████╗\r
░████║░░╚██╗░  ██╔════╝██╔══██╗██╔══██╗██║░░░░░╚██╗░██╔╝  ██║░░░░░██║██╔════╝██╔════╝\r
██╔██║░░░╚██╗  █████╗░░███████║██████╔╝██║░░░░░░╚████╔╝░  ██║░░░░░██║█████╗░░█████╗░░\r
╚═╝██║░░░██╔╝  ██╔══╝░░██╔══██║██╔══██╗██║░░░░░░░╚██╔╝░░  ██║░░░░░██║██╔══╝░░██╔══╝░░\r
███████╗██╔╝░  ███████╗██║░░██║██║░░██║███████╗░░░██║░░░  ███████╗██║██║░░░░░███████╗\r
╚══════╝╚═╝░░  ╚══════╝╚═╝░░╚═╝╚═╝░░╚═╝╚══════╝░░░╚═╝░░░  ╚══════╝╚═╝╚═╝░░░░░╚══════╝\r

        - Billy the Kid was born as Henry McCarty in NYC in 1859\r\n
        - Born to Catherine and Patrick McCarty soon having a younger brother named Joseph\r\n
        - His father suddenly passed away and Catherine took herself and her two sons to Indianapolis\r\n
        - During this time Catherine began a romance with a William Antrim\r\n
        - Soon after the four moved to Wichita, Kansas, in 1870\r\n
        - They then moved to Santa Fe, where Henry's mom would marry Antrim in 1873, and later that same year they moved to Silver City, New Mexico\r\n
        - A year later Henry's mom died of tuberculosis leaving Henry and Joseph in the care of their step-father\r\n
        - William would beat Henry and with the death of Henry's mother they became even worse as she wasn't there to protect her child.\r\n
        - Henry left home and began to live in a boarding house in which he would do work in exchange for living quarters.\r\n
        - He would make a very small amount of money here so he could hardly pay for proper food.\r\n"),
        ResetColor).ok();
}

pub fn third() {
    clearscreen::clear().expect("err");
    execute!(
        stdout(),
        SetForegroundColor(Color::White),
        SetBackgroundColor(Color::Black),
        Print("
██████╗░██╗░░  ███████╗██╗██████╗░░██████╗████████╗  ░█████╗░██████╗░██╗███╗░░░███╗███████╗░██████╗\r
╚════██╗╚██╗░  ██╔════╝██║██╔══██╗██╔════╝╚══██╔══╝  ██╔══██╗██╔══██╗██║████╗░████║██╔════╝██╔════╝\r
░░███╔═╝░╚██╗  █████╗░░██║██████╔╝╚█████╗░░░░██║░░░  ██║░░╚═╝██████╔╝██║██╔████╔██║█████╗░░╚█████╗░\r
██╔══╝░░░██╔╝  ██╔══╝░░██║██╔══██╗░╚═══██╗░░░██║░░░  ██║░░██╗██╔══██╗██║██║╚██╔╝██║██╔══╝░░░╚═══██╗\r
███████╗██╔╝░  ██║░░░░░██║██║░░██║██████╔╝░░░██║░░░  ╚█████╔╝██║░░██║██║██║░╚═╝░██║███████╗██████╔╝\r
╚══════╝╚═╝░░  ╚═╝░░░░░╚═╝╚═╝░░╚═╝╚═════╝░░░░╚═╝░░░  ░╚════╝░╚═╝░░╚═╝╚═╝╚═╝░░░░░╚═╝╚══════╝╚═════╝░\r

        - In 1875, Henry was caught stealing food,\r\n
        - Ten days later he and an accomplice, named Sombrero Jack, robbed a Chinese laundromat in which they took some clothes and two pistols.\r\n
        - Henry hid the goods in the boarding house but they were later found and Henry was reported to the police.\r\n
        - Henry was arrested and while in the jailhouse he was skinny enough to squeeze between the bars of his cell and he shimmied up a chimney to get to the roof and escape.\r\n
        - Henry fled to his step-father's house and after just a few days he was kicked out. Henry robbed his step-father and took serveral clothing items as well as several guns,\r\n
            marking the last time he and his step-father ever saw each other.\r\n
        - After this Henry began using the name Henry Antrim, which was likely to help him evade authorities.\r\n
        - Henry fled to  South-eastern Arizona, where he would become a ranch hand for Henry Hooker. He would typically blow all of his wages by gambling.\r\n
        - While working under Hooker, Henry met John Mackie, an ex-military cavalry private turned criminal, and they would steal horses from a nearby U.S. Army outpost, Camp Grant.\r\n
        - Due to Henry's boyish appearance and relatively young age compared to his outlaw peers he got the nickname, \"The Kid.\"\r\n
        - In 1877, Henry got into an argument with a blacksmith at a saloon who had a history of bullying Henry. During the argument the blacksmith called Henry a pimp, and in\r\n
            retaliation Henry called him a son of a bitch.\r\n
        - After this Henry was thrown to the ground and was getting the tar beat out of him. Henry tried to reach for the blacksmith's gun and the two began fighting over the gun,\r\n
            and eventually Henry got the gun in his hands and he shot and killed the blacksmith.\r\n
        - He fled the town though he returned a few days later, he was then arrested and held in Camp Grant. He again managed to slip through the bars and move to New Mexico after\r\n
            stealing a horse.\r\n
        - He was ambushed by Apache and had to walk the rest of the way to Fort Stanton, he turned up there nearly dead from dehydration and he was nursed back to health by\r\n
            John Jone's mother which was a friend he knew there.\r\n
        - Henry later travelled to Apache Tejo, an abandoned army post that was then occupied by a gang of cattle rustlers.\r\n
        - This is when Henry began using the alias, William H. Bonney. William was then shortened to Billy, giving Henry the now infamous nickname, Billy the Kid.\r\n"),
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
        Print("
░██████╗░█████╗░██╗░░░██╗██████╗░░█████╗░███████╗░██████╗\r
██╔════╝██╔══██╗██║░░░██║██╔══██╗██╔══██╗██╔════╝██╔════╝\r
╚█████╗░██║░░██║██║░░░██║██████╔╝██║░░╚═╝█████╗░░╚█████╗░\r
░╚═══██╗██║░░██║██║░░░██║██╔══██╗██║░░██╗██╔══╝░░░╚═══██╗\r
██████╔╝╚█████╔╝╚██████╔╝██║░░██║╚█████╔╝███████╗██████╔╝\r
╚═════╝░░╚════╝░░╚═════╝░╚═╝░░╚═╝░╚════╝░╚══════╝╚═════╝░\r

        \"Absolute Mad Lads - Billy the Kid\" YouTube, uploaded by Count Dankula, 13-12-2019, https://www.youtube.com/watch?v=QjWqfV3j-eY\"\r\n
        Garrett, Pat F. (1882). The Authentic Life of Billy, the Kid (1st ed.). Santa Fe, New Mexico: New Mexican Printing and Publishing Company. OCLC 748293298.\r\n
        [Billy the Kid photograph taken by Ben Wittick in 1879 or 1880.](https://commons.wikimedia.org/wiki/File:Billy_the_Kid_tintype,_Fort_Sumner,_1879-80-Edit2.jpg)"),
        ResetColor).ok();
}

pub fn err() {
    println!("This is not a valid value, please try again.");
}