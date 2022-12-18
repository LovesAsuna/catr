use clap::{arg, Arg, ArgAction, Command};
use std::io::{BufRead, BufReader};

pub fn run(config: Config) -> anyhow::Result<()> {
    dbg!(&config);
    for file_name in config.files {
        match open(&file_name) {
            Err(err) => eprintln!("Failed to open {}: {}", file_name, err),
            Ok(file) => {
                let mut last_num = 0;
                let end = if config.show_ends { "$" } else { "" };
                for (line_num, line) in file.lines().enumerate() {
                    let mut line = line?;
                    line = if config.show_tabs {
                        line.replace('\t', "^I")
                    } else {
                        line
                    };
                    if config.number_lines {
                        println!("{:>6}\t{}{}", line_num + 1, line, end);
                    } else if config.number_nonblank_lines {
                        if !line.is_empty() {
                            last_num += 1;
                            println!("{:>6}\t{}{}", last_num, line, end);
                        } else {
                            println!("{}", end);
                        }
                    } else {
                        println!("{}{}", line, end);
                    }
                }
            }
        }
    }
    Ok(())
}

#[derive(Debug)]
pub struct Config {
    files: Vec<String>,
    show_ends: bool,
    number_lines: bool,
    number_nonblank_lines: bool,
    show_tabs: bool,
}

pub fn get_args() -> anyhow::Result<Config> {
    let matches = Command::new("catr")
        .version("0.1.0")
        .author("LovesAsuna <qq625924077@gmail.com>")
        .about("Rust cat")
        .arg(
            Arg::new("files")
                .value_name("FILE")
                .help("Input file(s)")
                .action(ArgAction::Append),
        )
        .arg(
            Arg::new("show-ends")
                .short('E')
                .long("show-ends")
                .help("display $ at end of each line")
                .action(ArgAction::SetTrue),
        )
        .arg(
            Arg::new("number")
                .short('n')
                .long("number")
                .help("number all output lines")
                .action(ArgAction::SetTrue),
        )
        .arg(
            Arg::new("number-nonblank")
                .short('b')
                .long("number-nonblack")
                .help("numer nonempty output lines, overrides -n")
                .overrides_with("number")
                .action(ArgAction::SetTrue),
        )
        .arg(
            Arg::new("show-tabs")
                .short('T')
                .long("show-tabs")
                .help("display TAB characters as ^I")
                .action(ArgAction::SetTrue),
        )
        .get_matches();
    Ok(Config {
        files: matches
            .get_many::<String>("files")
            .unwrap_or_default()
            .map(|s| s.to_string())
            .collect(),
        show_ends: matches.get_flag("show-ends"),
        number_lines: matches.get_flag("number"),
        number_nonblank_lines: matches.get_flag("number-nonblank"),
        show_tabs: matches.get_flag("show-tabs"),
    })
}

fn open(file_name: &str) -> anyhow::Result<Box<dyn BufRead>> {
    match file_name {
        "-" => Ok(Box::new(BufReader::new(std::io::stdin()))),
        _ => Ok(Box::new(BufReader::new(std::fs::File::open(file_name)?))),
    }
}
