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
        6 => seventh(),
        7 => eighth(),
        8 => ninth(),
        9 => sources(),
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
 ╚══════╝╚═╝░░  ╚══════╝╚═╝░░╚═╝╚═╝░░╚═╝╚══════╝░░░╚═╝░░░  ╚══════╝╚═╝╚═╝░░░░░╚══════╝\r\n"),ResetColor
    ).ok();
    sleep(Duration::from_secs(2));
    execute!(
        stdout(),
        SetForegroundColor(Color::White),
        SetBackgroundColor(Color::Black),
        Print("
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
 ╚══════╝╚═╝░░  ╚═╝░░░░░╚═╝╚═╝░░╚═╝╚═════╝░░░░╚═╝░░░  ░╚════╝░╚═╝░░╚═╝╚═╝╚═╝░░░░░╚═╝╚══════╝╚═════╝░\r\n"),ResetColor
    ).ok();
    sleep(Duration::from_secs(2));
    execute!(
        stdout(),
        SetForegroundColor(Color::White),
        SetBackgroundColor(Color::Black),
        Print("
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
        Print("
 ██████╗░██╗░░                  ██╗░░░░░██╗███╗░░██╗░█████╗░░█████╗░██╗░░░░░███╗░░██╗  ░█████╗░░█████╗░██╗░░░██╗███╗░░██╗████████╗██╗░░░██╗  ░██╗░░░░░░░██╗░█████╗░██████╗░\r
 ╚════██╗╚██╗░                  ██║░░░░░██║████╗░██║██╔══██╗██╔══██╗██║░░░░░████╗░██║  ██╔══██╗██╔══██╗██║░░░██║████╗░██║╚══██╔══╝╚██╗░██╔╝  ░██║░░██╗░░██║██╔══██╗██╔══██╗\r
 ░█████╔╝░╚██╗  █▀█ █▀█ █▀▀ ▄▄  ██║░░░░░██║██╔██╗██║██║░░╚═╝██║░░██║██║░░░░░██╔██╗██║  ██║░░╚═╝██║░░██║██║░░░██║██╔██╗██║░░░██║░░░░╚████╔╝░  ░╚██╗████╗██╔╝███████║██████╔╝\r
 ░╚═══██╗░██╔╝  █▀▀ █▀▄ ██▄ ░░  ██║░░░░░██║██║╚████║██║░░██╗██║░░██║██║░░░░░██║╚████║  ██║░░██╗██║░░██║██║░░░██║██║╚████║░░░██║░░░░░╚██╔╝░░  ░░████╔═████║░██╔══██║██╔══██╗\r
 ██████╔╝██╔╝░                  ███████╗██║██║░╚███║╚█████╔╝╚█████╔╝███████╗██║░╚███║  ╚█████╔╝╚█████╔╝╚██████╔╝██║░╚███║░░░██║░░░░░░██║░░░  ░░╚██╔╝░╚██╔╝░██║░░██║██║░░██║\r
 ╚═════╝░╚═╝░░                  ╚══════╝╚═╝╚═╝░░╚══╝░╚════╝░░╚════╝░╚══════╝╚═╝░░╚══╝  ░╚════╝░░╚════╝░░╚═════╝░╚═╝░░╚══╝░░░╚═╝░░░░░░╚═╝░░░  ░░░╚═╝░░░╚═╝░░╚═╝░░╚═╝╚═╝░░╚═╝\r\n"),ResetColor
    ).ok();
    sleep(Duration::from_secs(2));
    execute!(
        stdout(),
        SetForegroundColor(Color::White),
        SetBackgroundColor(Color::Black),
        Print("
        - Billy found work as a rancher for John Henry Tunstall, attempting to become a cattle magnate and merchant in Lincoln County along with his partner, Alexander McSween.\r\n
        - The problem with this being that there was already a cattle and merchant outfit in Lincoln County, ran by two men called Lawrence Murphy and James Dolan.\r\n
            This outfit was named \"The House.\"\r\n
        - Tunstall would constantly clash with The House because they were trying to push him out and maintain their monopoly by buying up all the resources and even threatening\r\n
            Tunstall's clients.\r\n
        - This tension would continue and an event that affected the two sellers would snowball into what would later be called the Lincoln County War.\r\n
        - In 1878, Emil Fritz passed away and he had a life insurance policy along with a will which would decide to whom the money would go to. \r\n
        - McSween, being a lawyer, was hired to manage the accounts and payout to whom was to be payed.\r\n
        - Upon review McSween noticed that everything in Fritz's name was to go to Lawrence Murphy. He was not too fond of the idea of giving his business competitor who had\r\n
            repeatedly harrassed both McSween, Tunstall, and his patrons. So, McSween refused to pay Murphy.\r\n
        - Murphy obtained a court order that allowed them to seize assets from McSween to gather the money that he was supposed to be payed. This court order, however, did not say\r\n
            they could only seize McSween's assets, but Tunstall's as well. \r\n
        - The House saw this as a golden opportunity to go into their business and take anything and everything they wanted to prevent their business from being able to compete with them.\r\n
        - Eventually, to stop Tunstall's cattle from being constantly stolen, he and his ranchers gathered their cattle to move them to a hidden place in which the gangs that The House\r\n
            had hired could not find it.\r\n
        - What Tunstall didn't know was that a possee was formed to seize the cattle in the name of The House, lead by William J. Brady--the sheriff.\r\n
        - When Tunstall saw the possee on his land he told his ranchers, including Billy the Kid, to stay with the cattle and that he would go see the possee by himself.\r\n
        - So they stayed, as Tunstall was approaching them he was shot in the chest and he fell from his horse. As Tunstall laid on the ground one of the members of the possee walked\r\n
            over to him and took his gun, he then shot Tunstall in the head with his own gun. All of this in full view of Billy and the other ranchers.\r\n
        - The ranchers swore they would get revenge on The House and sheriff Brady for murdering John Tunstall.\r"),
        ResetColor).ok();
}
pub fn fifth() {
    clearscreen::clear().expect("err");
    execute!(
        stdout(),
        SetForegroundColor(Color::White),
        SetBackgroundColor(Color::Black),
         Print(" ░░██╗██╗██╗░░  ██╗░░░░░██╗███╗░░██╗░█████╗░░█████╗░██╗░░░░░███╗░░██╗  ░█████╗░░█████╗░██╗░░░██╗███╗░░██╗████████╗██╗░░░██╗  ░██╗░░░░░░░██╗░█████╗░██████╗░\r
 ░██╔╝██║╚██╗░  ██║░░░░░██║████╗░██║██╔══██╗██╔══██╗██║░░░░░████╗░██║  ██╔══██╗██╔══██╗██║░░░██║████╗░██║╚══██╔══╝╚██╗░██╔╝  ░██║░░██╗░░██║██╔══██╗██╔══██╗\r
 ██╔╝░██║░╚██╗  ██║░░░░░██║██╔██╗██║██║░░╚═╝██║░░██║██║░░░░░██╔██╗██║  ██║░░╚═╝██║░░██║██║░░░██║██╔██╗██║░░░██║░░░░╚████╔╝░  ░╚██╗████╗██╔╝███████║██████╔╝\r
 ███████║░██╔╝  ██║░░░░░██║██║╚████║██║░░██╗██║░░██║██║░░░░░██║╚████║  ██║░░██╗██║░░██║██║░░░██║██║╚████║░░░██║░░░░░╚██╔╝░░  ░░████╔═████║░██╔══██║██╔══██╗\r
 ╚════██║██╔╝░  ███████╗██║██║░╚███║╚█████╔╝╚█████╔╝███████╗██║░╚███║  ╚█████╔╝╚█████╔╝╚██████╔╝██║░╚███║░░░██║░░░░░░██║░░░  ░░╚██╔╝░╚██╔╝░██║░░██║██║░░██║\r
 ░░░░░╚═╝╚═╝░░  ╚══════╝╚═╝╚═╝░░╚══╝░╚════╝░░╚════╝░╚══════╝╚═╝░░╚══╝  ░╚════╝░░╚════╝░░╚═════╝░╚═╝░░╚══╝░░░╚═╝░░░░░░╚═╝░░░  ░░░╚═╝░░░╚═╝░░╚═╝░░╚═╝╚═╝░░╚═╝\r\n"),ResetColor
    ).ok();
    sleep(Duration::from_secs(2));
    execute!(
        stdout(),
        SetForegroundColor(Color::White),
        SetBackgroundColor(Color::Black),
        Print("
        - Billy the Kid and his associate Dick Brewer swore affidavits against Brady and his possee to justice of the peace, John B. Wilson, saying that they had witnessed him\r
            commit a murder and that he had gone rogue and was abusing his powers as sheriff.\r\n
        - Wilson then deputised Billy, Dick Brewer, and the rest of the ranchers. Then issued murder warrants for the murderers of Tunstall.\r\n
        - Billy and the ranchers created a possee called The Regulators consisting of the now deputised ranchers.\r\n
        - In February of 1878 Billy and Brewer saw Brady and his men, so they rode up to them and told them that they were deputised and that they were there to arrest them.\r\n
        - Brady and his men said, \"No you,\" and arrested Billy and Brewer. During the arrest Brady confiscated Billy's treasured Winchester rifle.\r\n
        - Billy was typically known for being level-headed but he was absolutely furious that Brady had stolen his rifle.\r\n
        - Later The Regulators had broken Billy and Brewer out of the jail and put the guards into the cells.\r\n
        - They then rode out into the wilderness to track down Brady and his possee.\r\n
        - They later found 3 members of the possee and they began to chase them for 8km (~5m) gun fighting the entire way. Eventually the 3 surrendered.\r\n
        - One of the 3 prisoners were friends with one of The Regulators, the prisoner being Buck Morton and the other being William McCloskey.\r\n
        - Due to this friendship William prevented the other Regulators from killing the three prisoners.\r\n
        - As they were coming back they reached Black Water Creek and, according to The Regulators, Buck was somehow able to grab someone's gun and he shot and killed McCloskey,\r
            the 3 prisoners then tried to escape, leaving them no choice but to gun down the prisoners.\r\n
        - This event would later become known as the Black Water Massacre.\r\n
        - Later that day another group of Regulators caught up to 2 members of Brady's possee and a gun fight ensued. Out of the two, one was killed and the other was severely wounded\r
            and captured.\r\n
        - Brady began to panic as he was actually losing to The Regulators. So he wrote to the territorial governor and lied about the situation.\r\n
        - This resulted in Wilson, the justice of the peace who had deptutised The Regulators, being removed from office which made all of his decrees invalid.\r\n
            This made it so that Brady and his possee were the only recognized law in Lincoln County.\r\n
        - The Regulators' actions up until this point had been legal as they were deputised, but from this point on, if they continued on they would be considered wanted men.\r\n
        "),
        ResetColor
    ).ok();
}
pub fn sixth() {
    clearscreen::clear().expect("err");
    execute!(
        stdout(),
        SetForegroundColor(Color::White),
        SetBackgroundColor(Color::Black),
        Print("
 ███████╗██╗░░  ░█████╗░░█████╗░███╗░░██╗████████╗░░░\r
 ██╔════╝╚██╗░  ██╔══██╗██╔══██╗████╗░██║╚══██╔══╝░░░\r
 ██████╗░░╚██╗  ██║░░╚═╝██║░░██║██╔██╗██║░░░██║░░░░░░\r
 ╚════██╗░██╔╝  ██║░░██╗██║░░██║██║╚████║░░░██║░░░░░░\r
 ██████╔╝██╔╝░  ╚█████╔╝╚█████╔╝██║░╚███║░░░██║░░░██╗\r
 ╚═════╝░╚═╝░░  ░╚════╝░░╚════╝░╚═╝░░╚══╝░░░╚═╝░░░╚═╝\r\n"),
        ResetColor
    ).ok();
    sleep(Duration::from_secs(2));
    execute!(
        stdout(),
        SetForegroundColor(Color::White),
        SetBackgroundColor(Color::Black),
        Print("
        - One day Brady and his possee were walking down the main street of Lincoln, but what they didn't know is that hidden around the street were members of The Regulators,\r
            including Billy the kid.\r\n
        - When a signal was given, all of the regulators jumped out and opened fire on Brady and his men. Brady was hit over a dozen times and was killed instantly.\r\n
        - As the gun fight ensued Billy ran to Brady's corpse to steal back his Winchester rifle, as he was running back however he was hit in the thigh.\r\n
        - Despite all of this, all of The Regulators managed to get away.\r\n
        - A few days later they began to head for a place called Blazer's Mill because they got word that one of Tunstall's murderers was hiding out there, a man called\r\n
            Buckshot Roberts.\r
        - As The Regulators approached they were spotted by Buckshot and opened fire, starting a gun fight.\r\n
        - Buckshot was killed during the fight, however he managed to wound 4 of The Regulators and he killed Dick Brewer.\r\n
        - Since Sheriff Brady had died, another was to be appointed in his place. A man named John Copland, however as a sheriff he refused to take sides during the war.\r\n
        - Because of this The House used their influence and power to replace him with someone who would be on their side, a man named George Peppin.\r\n
        - In July 1978, The Regulators got word that Peppin and his possee were coming to kill McSween who was at his home in Lincoln.\r\n
        - This lead to 60 Regulators descending on the town and taking up defensive positions around the town but mainly around McSween's home.\r\n
        - Peppin and his men arrived and engaged The Regulators, kicking off a gun fight that would last for 5 days, later dubbed The Battle of Lincoln\r\n
        - Billy the Kid was only around 18 at this time and because of this battle many people began to fear BIlly more as it showed he was a very strong marksman,\r\n
        - There were very heavy losses from both sides, but what The Regulators didn't know is that Sheriff Peppin called in for backup from the U.S. Army,\r
            and when they came, they came with force.\r\n
        - When The Regulators saw the army coming, they took refuge in Tunstall's store and McSween's house, until the army began shooting these buildings with cannons.\r\n
        - Peppin and his men also set fire to McSween's house, leaving the inhabitants to escape only through the backdoor.\r\n
        - While escaping, McSween was shot and killed by a man named Robert Beckwith, a man who was immediately shot and killed by Billy the Kid.\r\n
        - Most of The Regulators were either killed or captured, they lost the Battle of Lincoln.\r\n
        - The rest had retreated into the wilderness to live as outlaws.\r\n"),
        ResetColor
    ).ok();
}

pub fn seventh() {
    clearscreen::clear().expect("err");
    execute!(
        stdout(),
        SetForegroundColor(Color::White),
        SetBackgroundColor(Color::Black),
        Print("
 ░█████╗░██╗░░  ░█████╗░███████╗████████╗███████╗██████╗░  ████████╗██╗░░██╗███████╗  ░██╗░░░░░░░██╗░█████╗░██████╗░\r
 ██╔═══╝░╚██╗░  ██╔══██╗██╔════╝╚══██╔══╝██╔════╝██╔══██╗  ╚══██╔══╝██║░░██║██╔════╝  ░██║░░██╗░░██║██╔══██╗██╔══██╗\r
 ██████╗░░╚██╗  ███████║█████╗░░░░░██║░░░█████╗░░██████╔╝  ░░░██║░░░███████║█████╗░░  ░╚██╗████╗██╔╝███████║██████╔╝\r
 ██╔══██╗░██╔╝  ██╔══██║██╔══╝░░░░░██║░░░██╔══╝░░██╔══██╗  ░░░██║░░░██╔══██║██╔══╝░░  ░░████╔═████║░██╔══██║██╔══██╗\r
 ╚█████╔╝██╔╝░  ██║░░██║██║░░░░░░░░██║░░░███████╗██║░░██║  ░░░██║░░░██║░░██║███████╗  ░░╚██╔╝░╚██╔╝░██║░░██║██║░░██║\r
 ░╚════╝░╚═╝░░  ╚═╝░░╚═╝╚═╝░░░░░░░░╚═╝░░░╚══════╝╚═╝░░╚═╝  ░░░╚═╝░░░╚═╝░░╚═╝╚══════╝  ░░░╚═╝░░░╚═╝░░╚═╝░░╚═╝╚═╝░░╚═╝\r\n"),
        ResetColor
    ).ok();
    sleep(Duration::from_secs(2));
    execute!(
        stdout(),
        SetForegroundColor(Color::White),
        SetBackgroundColor(Color::Black),
        Print("
        - In 1878, U.S. Martial John Sherman appealed to the newly appointed territorial governor for assistance, stating that the Lincoln County War had thrown the\r\n
            entire county into dissaray and anarchy and that he was having trouble managing the county and carrying out his duties.\r\n
        - So the territorial governor issued a decree, stating that anyone who participated in the Lincoln County War would be pardoned. As everyone learned of this,\r\n
            killings stopped, people came out of hiding, and peace was restored to Lincoln.\r\n
        - Though this decree had a few exceptions, for example, anyone who was convicted or indicted of an offense during the Lincoln County War their warrants and\r\n
            convictions would still be valid and would still stand.\r\n
        - Billy the Kid, having a warrant issued to him for the murder of William Brady, would not be pardoned.\r\n
        - In 1879, Billy and a friend came back into Lincoln on business and they were forced to watch Jesse Evans, leader of the Jesse Evans gang, murder a man named\r\n
            Huston Chapman and then set his body on fire at gunpoint.\r\n
        - Billy then wrote to the Governor Wallace telling him that he was witness to the murder and that he was willing to testify against Jesse Evans and the gang in\r\n
            exchange for a full pardon, immunity, and protection from retribution from the Jesse Evans gang.\r\n
        - They wrote back and forth many times and even met up at one point to discuss these terms, eventually the governor agreed to his terms.\r\n
        - It was arranged that an unbiased sheriff would bring Billy in, they then met at a secure location and Billy was brought to a jailhouse for his safety.\r\n
        - He then testified against Jesse Evans in front of a grand jury, and was taken back to the jailhouse to keep him safe.\r\n
        - Billy upheld his end of the bargain, but the jailhouse was refusing to let him free.\r\n
        - Billy waited for a few weeks, writing to the governor reminding him that he was supposed to be set free.\r\n
        - After realizing the governor was lying and trying to capture him, Billy did his usual and slipped through the bars and escaped.\r\n
        - Billy was once again on the run, however this time he had a $500 bounty on his head.\r"),
        ResetColor
    ).ok();
}

pub fn eighth() {
    clearscreen::clear().expect("err");
    execute!(
        stdout(),
        SetForegroundColor(Color::White),
        SetBackgroundColor(Color::Black),
        Print("
 ███████╗██╗░░  ░█████╗░░█████╗░███╗░░██╗████████╗░░░\r
 ╚════██║╚██╗░  ██╔══██╗██╔══██╗████╗░██║╚══██╔══╝░░░\r
 ░░░░██╔╝░╚██╗  ██║░░╚═╝██║░░██║██╔██╗██║░░░██║░░░░░░\r
 ░░░██╔╝░░██╔╝  ██║░░██╗██║░░██║██║╚████║░░░██║░░░░░░\r
 ░░██╔╝░░██╔╝░  ╚█████╔╝╚█████╔╝██║░╚███║░░░██║░░░██╗\r
 ░░╚═╝░░░╚═╝░░  ░╚════╝░░╚════╝░╚═╝░░╚══╝░░░╚═╝░░░╚═╝\r\n"),
        ResetColor
    ).ok();
    sleep(Duration::from_secs(2));
    execute!(
        stdout(),
        SetForegroundColor(Color::White),
        SetBackgroundColor(Color::Black),
        Print("
        - He laid low until January 1880 when he was hiding out at Fort Sumner and a man kept staring at him, a man named Joe Grant. Billy was informed that\r\n
            Grant was a bounty hunter, so Billy knew he had to do something about the situation.\r\n
        - So, naturally, Billy walked up to Grant, complemented his gun and asked if he could see it. Puzzled, Grant obliged and handed his gun to Billy. \r\n
        - This is when Billy noticed that Grant's gun only had 3 rounds inside of it, so Billy turned the cylinder so that the next hammerfall would land\r\n
            on an empty chamber. He then handed it back to Grant and he reholstered it.\r\n
        - Billy then started talking smack to Grant and making fun of him until Grant pulled out his gun and pointed it at Billy's face.\r\n
        - Billy then immediately drew his own gun and shot Grant in the head.\r\n
        - When asked about this event later he said, \"It was a game of two, and I got there first.\"\r\n
        - Later that year Billy and other outlaws were hiding out at a place called Great House Ranch, when the ranch was surrounded by a possee lead by\r\n
            Sheriff James Carlisle.\r\n
        - Billy and the outlaws were friend with the owner of the ranch but they yelled that they were holding him hostage. \r\n
        - Carlisle offered to swap places with Great House and Billy agreed, likely to get Great House to safety.\r\n
        - Carlisle tried to escape through a window but was shot dead. The shootout ended when the possee was forced to withdraw and Billy and the others escaped.\r\n
        - They attempted to return to Fort Sumner but they had a possee waiting for them, lead by Sheriff Pat Garrett. In the ensuing shootout one of Billy's\r\n
            men died however the rest and himself managed to get away.\r\n
        - Sheriff Garrett kept hunting Billy and they managed to get the drop on him and captured him along with his men. They were taken to Las Vegas and he\r\n
            was to take them to Santa Fe the next day for trial.\r\n"),
        ResetColor
    ).ok();
}

pub fn ninth() {
    clearscreen::clear().expect("err");
    execute!(
        stdout(),
        SetForegroundColor(Color::White),
        SetBackgroundColor(Color::Black),
        Print("
 █▀█ ▀▄   █▀▀ █▀█ █▄░█ ▀█▀ ░\r
 ▀▀█ ▄▀   █▄▄ █▄█ █░▀█ ░█░ ▄\r\n"),
        ResetColor
    ).ok();
    sleep(Duration::from_secs(2));
    execute!(
        stdout(),
        SetForegroundColor(Color::White),
        SetBackgroundColor(Color::Black),
        Print("
        - However, an armed crowd had formed at the train station demanding that Garrett hand over Dave Rudabaugh as he had killed a local jailer and they wanted\r
            to execute him in their jurisdiction.\r\n
        - There was a heated back and forth until it was decided that Dave could go to Santa Fe as long as he was later given to the townsfolk so that they could execute him.\r\n
        - Billy was quoted as saying, \"If only I had my Winchester, I'd lick the whole crowd.\"\r\n
        - Billy was taken to Santa Fe jail and wrote governor Wallace asking him to uphold his promise, but he refused.\r\n
        - He was tried and sentenced to death for the murder of Sheriff Brady.\r\n
        - During the sentencing the judge said, \"You will hang until you're dead, dead, dead.\" Billy responded, \"Well you can go to hell, hell, hell.\"\r\n
        - Billy was kept under high security in courthouse jail til he could be executed.\r\n
        - One day, Billy was left with one guard looking over him as the others were brought to get a bite to eat at the local saloon.\r\n
        - Billy asked James if he could be taken to use the outhouse, James obliged and took a shackled Billy to the outhouse. Billy did his business and left.\r\n
        - As he was returning he purposefully walked fast to create distance between him and James and as he reached the top of the courthouse stairs he ducked\r
            behind a corner and slipped out of his handcuffs.\r\n
        - As James reached the top of the stairs Billy attacked him and began beating him with the blunt end of the cuffs.\r\n
        - This soon turned to a scuffle for James' gun, which Billy eventually won. James attempted to escape but Billy shot him in the back, killing him.\r\n
        - Billy then went into the courthouse and stole a shotgun, he waited at the upstairs window knowing the other guard would have heard the gunshot and\r
            would be coming to investigate.\r\n
        - As he came closer and began to walk up to the courthouse Billy shouted from the window, \"Look up old boy and see what you get!\"\r\n
        - He looked up and Billy pulled the trigger and blasted him with the shotgun, he was killed instantly.\r\n
        - He then found a hatchet and used it to break his leg restraints, then stole a horse and rode out of town.\r\n
        - The sheriff got word that Billy was back at Fort Sumner, so he and two of his deputies came to Billy's friend's Maxwell's house to interrogate him.\r\n
        - Sheriff Garrett and Maxwell were sitting in their room and Billy came back to the property, he walked down the hall and looked into Maxwell's room,\r
            right at Garrett.\r\n
        - The house was poorly lit, so neither of them could make out the other, but Billy knew that it wasn't Maxwell.\r\n
        - Billy began to back away asking, in Spanish, who it was. Garrett recognized Billy's voice and immediately drew his gun, shooting at Billy twice and\r
            the first of which hit him in the heart. Billy fell to the floor, dead at the age of 21.\r"),
        ResetColor
    ).ok();
}

pub fn goodbye() {
    clearscreen::clear().expect("err");
    execute!(
        stdout(),
        SetForegroundColor(Color::White),
        SetBackgroundColor(Color::Black),
        Print(" Thank you for listening to my presentation."),
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