extern crate regex;
extern crate libc;
extern crate clap;

mod tty;

use std::io::BufReader;
use regex::Regex;
use std::process::{Command, Stdio};
use std::io::BufRead;
use clap::{App, AppSettings, Arg};

const CARGO: &'static str = "cargo";
const CRATE_LEN: usize = 26; // 99% of crates have names <= 26 chars at time of writing
const VERSION_LEN: usize = 6; // Typical form of 'vX.Y.Z'

fn main() {
	let matches = App::new("cargo-urlcrate")
        .setting(AppSettings::TrailingVarArg)
		.version("0.1")
		.about("Adds crate URLs to Cargo output")
		.author("Aaron1011")
		.arg(Arg::with_name("tty")
             .long("tty")
             .takes_value(true)
             .possible_values(&["never", "auto"])
             .default_value("auto")
             .help("On Linux, don't connect cargo to a tty. This will cause cargo to not emit colors with '--color auto' (the default). This option has no effect on non-Linux platforms")
        )
        .arg(Arg::with_name("cargo")
             .last(true)
             .multiple(true)
        )
		.get_matches();

    let raw = matches.value_of("cargo").unwrap_or("").to_owned();
    println!("Raw: {}", raw);

    run(raw, match matches.value_of("tty").unwrap() {
        "never" => false,
        "auto" => true,
        _ => unreachable!()
    });
}

fn run(raw: String, tty: bool) {
    // https://www.npmjs.com/package/ansi-regex
    let regex = Regex::new("[\u{001b}\u{009b}][\\[()#;?]*(?:[0-9]{1,4}(?:;[0-9]{0,4})*)?[0-9A-PRZcf-nqry=><]").unwrap();

    let mut command = Command::new(CARGO);
    command.args(raw.split(" "))
        .stdout(Stdio::inherit());

    let mut reader: BufReader<tty::Handle> = BufReader::with_capacity(10, tty::get_handle(command, tty));
    let newline: &[_] = &['\n', '\r'];
    loop {
        let mut line = String::new();
        let line = match reader.read_line(&mut line) {
            Ok(_) => line.trim_right_matches(newline),
            Err(e) => {
                if tty::handle_err(reader.get_mut(), e) { break } else { continue };
            }
        };
        let tmp = regex.replace_all(&line, "");

        let clean = tmp.trim_left();
        let split: Vec<_> = clean.split(" ").collect();

        match split[0].as_ref() {
            "Compiling" | "Downloading" => {
                let padding = (CRATE_LEN + VERSION_LEN).saturating_sub(split[1].len() + 1 + split[2].len());
                println!("{}{}https://crates.io/crates/{}", line, " ".repeat(padding), split[1]);
            }
            _ => println!("{}", line)
        }
    }
}


