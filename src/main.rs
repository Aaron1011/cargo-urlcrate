extern crate regex;
extern crate libc;

mod tty;

use std::io::BufReader;
use regex::Regex;
use std::process::{Command, Stdio};
use std::io::BufRead;

const CARGO: &'static str = "cargo";
const CRATE_LEN: usize = 26; // 99% of crates have names <= 26 chars at time of writing
const VERSION_LEN: usize = 6; // Typical form of 'vX.Y.Z'

fn main() {
    // https://www.npmjs.com/package/ansi-regex
    let regex = Regex::new("[\u{001b}\u{009b}][\\[()#;?]*(?:[0-9]{1,4}(?:;[0-9]{0,4})*)?[0-9A-PRZcf-nqry=><]").unwrap();

    //let args = vec!["--color=always".to_owned()];
    let args = vec![];

    let mut command = Command::new(CARGO);
    command.args(std::env::args().skip(1).chain(args.into_iter()))
        .stdout(Stdio::inherit());

    let mut reader: BufReader<tty::Handle> = BufReader::with_capacity(10, tty::get_handle(command));
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


