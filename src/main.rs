#![cfg_attr(feature="clippy", feature(plugin))]
#![cfg_attr(feature="clippy", plugin(clippy))]

extern crate regex;
extern crate libc;

#[macro_use]
extern crate serde_derive;
extern crate docopt;

mod tty;

use std::io::BufReader;
use regex::Regex;
use std::process::{Command, Stdio};
use std::io::BufRead;
use docopt::Docopt;

const CARGO: &'static str = "cargo";
const CRATE_LEN: usize = 26; // 99% of crates have names <= 26 chars at time of writing
const VERSION_LEN: usize = 6; // Typical form of 'vX.Y.Z'

const USAGE: &'static str =  "
Adds crate URLS to Cargo output

Usage:
    cargo-urlcrate [options]
    cargo-urlcrate [options] [--] <args>...

Options:
    -h, --help          Display this message
    -V, --version       Print version info and exit
    --tty=MODE          Allowed modes: auto/never. Determines whether 'cargo' is attached to a tty. Ignored on platforms other than linux.

When Cargo's color mode is set to 'auto' ('--color auto'), it determines if its stderr is attached to a tty or a pipe.
cargo-urlcrate supports connecting a psuedo-terminal (pty) to cargo, which will cause Cargo to detect a terminal.

This option only has an effect on Linux - other platforms can pass '--color always' to Cargo to force it to output colors anyway.
";

#[derive(Debug, Deserialize)]
struct Args {
    flag_help: bool,
    flag_version: bool,
    flag_tty: Option<Mode>,
    arg_args: Vec<String>
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "lowercase")]
enum Mode {
    Auto,
    Never
}

fn main() {
    // The program is invoked like this: 'cargo urlcrate [args]'
    // We need to skip over the second arg, but include the first arg and any args after the second
    // arg

    let parsed = std::env::args().take(1).chain(std::env::args().skip(2)).collect::<Vec<String>>();
    let args: Args = Docopt::new(USAGE)
        .map(|d| d.argv(parsed))
        .and_then(|d| d.deserialize()).unwrap_or_else(|e| e.exit());

    if args.flag_version {
        println!("cargo-urlcrate v0.1.0");
        return
    }

    run(args.arg_args, match args.flag_tty.unwrap_or(Mode::Auto) {
        Mode::Never => false,
        Mode::Auto => true,
    });
}

fn run(args: Vec<String>, tty: bool) {
    // https://www.npmjs.com/package/ansi-regex
    let regex = Regex::new("[\u{001b}\u{009b}][\\[()#;?]*(?:[0-9]{1,4}(?:;[0-9]{0,4})*)?[0-9A-PRZcf-nqry=><]").unwrap();

    let mut command = Command::new(CARGO);
    command.args(args)
        .stdout(Stdio::inherit());

    let mut reader: BufReader<tty::Handle> = BufReader::with_capacity(10, tty::get_handle(command, tty));
    let newline: &[_] = &['\n', '\r'];
    loop {
        let mut line = String::new();
        let line = match reader.read_line(&mut line) {
            Ok(num) => {
                if num == 0 {
                    break
                }
                line.trim_right_matches(newline)
            },
            Err(e) => {
                if tty::handle_err(reader.get_mut(), &e) { break } else { continue };
            }
        };
        let clean = regex.replace_all(line, "");
        let split: Vec<_> = clean.trim_left().split(' ').collect();

        match split[0].as_ref() {
            "Compiling" | "Downloading" => {
                let padding = (CRATE_LEN + VERSION_LEN).saturating_sub(split[1].len() + 1 + split[2].len());
                eprintln!("{}{}https://crates.io/crates/{}", line, " ".repeat(padding), split[1]);
            }
            _ => eprintln!("{}", line)
        }
    }
}


