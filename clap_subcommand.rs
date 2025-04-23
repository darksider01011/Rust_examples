use clap::{Arg,  Command, value_parser};

fn main() {
    let matches = Command::new("myapp").about("PicoDuckey Payload Generator")
        .subcommand(
            Command::new("w7")
                .about("Windows 7 Payload Generator")
                .arg(
                    Arg::new("wordlist")
                        .short('w')
                        .long("wordlist")
                        .required(true)
                        .value_parser(value_parser!(String))
                        .help("Set Wordlist"),
                )
                .arg(
                    Arg::new("delay")
                        .short('d')
                        .long("delay")
                        .value_parser(value_parser!(String))
                        .help("Set Delay Time In Seconds")
                        .default_value("20"),),
        )
        .subcommand(
            Command::new("w8")
                .about("Windows 8 Payload Generator")
                .arg(
                    Arg::new("wordlist")
                        .short('w')
                        .long("wordlist")
                        .required(true)
                        .value_parser(value_parser!(String))
                        .help("Set Wordlist"),
                )
                .arg(
                    Arg::new("delay")
                        .short('d')
                        .long("delay")
                        .value_parser(value_parser!(String))
                        .help("Set Delay Time In Seconds")
                        .default_value("20"),),
        )
        .subcommand(
            Command::new("w10")
                .about("Windows 10 Payload Generator")
                .arg(
                    Arg::new("wordlist")
                        .short('w')
                        .long("wordlist")
                        .required(true)
                        .value_parser(value_parser!(String))
                        .help("Set Wordlist"),
                )
                .arg(
                    Arg::new("delay")
                        .short('d')
                        .long("delay")
                        .value_parser(value_parser!(String))
                        .help("Set Delay Time In Seconds")
                        .default_value("20"),),
        )
        .subcommand(
            Command::new("w11")
                .about("Windows 11 Payload Generator")
                .arg(
                    Arg::new("wordlist")
                        .short('w')
                        .long("wordlist")
                        .required(true)
                        .value_parser(value_parser!(String))
                        .help("Set Wordlist"),
                )
                .arg(
                    Arg::new("delay")
                        .short('d')
                        .long("delay")
                        .value_parser(value_parser!(String))
                        .help("Set Delay Time In Seconds")
                        .default_value("20"),),
        ).get_matches();

if let Some(sub_matches) = matches.subcommand_matches("w7") {
    let wordlist = sub_matches.get_one::<String>("wordlist").unwrap();
    let delay = sub_matches.get_one::<String>("delay").unwrap();
    println!("Delay: {}", delay);
    let greeting = format!("wordlist: {}!", wordlist);
    println!("{}", greeting);}
}
