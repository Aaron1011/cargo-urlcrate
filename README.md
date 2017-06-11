# cargo-urlcrate

[![Crate Version](https://img.shields.io/crates/v/cargo-urlcrate.svg)](https://crates.io/crates/cargo-urlcrate)
[![Travis status](https://travis-ci.org/Aaron1011/cargo-urlcrate.svg?branch=master)](https://travis-ci.org/Aaron1011/cargo-urlcrate)
[![AppVeyor status](https://ci.appveyor.com/api/projects/status/t0ooyuawtpciodl9?svg=true)](https://ci.appveyor.com/project/Aaron1011/cargo-urlcrate)
[![license-image](https://img.shields.io/badge/license-MIT-blue.svg)]()

A tool to add crate URLs to Cargo's output.

Bored waiting for a crate's dependencies to compile? cargo-urlcrate makes it easy to read about interesting crates while you wait.

### Installation
`cargo-urlcrate` is a Cargo subcommand, and can be installed with `cargo install`:

```
$ cargo install cargo-urlcrate
```

It can then be run as `cargo urlcrate [subcommand]` from any Cargo project

### Usage

Simply prefix any normal Cargo subcommand, such as `build` or `run`, with `cargo urlcrate`.

For example:

```
$ cargo urlcrate build
```

### Sample Output

[![](https://i.imgur.com/TOGF9IK.jpg)]()
