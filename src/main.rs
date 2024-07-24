use std::{fs::{self, ReadDir}, path::PathBuf};

use clap::{value_parser, Arg, ArgAction, Command, ValueHint};

fn main() {
    let matches = Command::new("Ls_Media")
        .version(env!("CARGO_PKG_VERSION"))
        .about("ls but for madia files")
        // path
        .arg(
            Arg::new("path")
                .action(ArgAction::Set)
                .value_name("PATH")
                .value_hint(ValueHint::DirPath)
                .value_parser(value_parser!(PathBuf))
                .index(1),
        )
        // long
        .arg(
            Arg::new("long")
                .action(ArgAction::SetTrue)
                .long("long")
                .short('l')
                .help("Display extended info"),
        )
        .get_matches();

    // target dir
    let path = matches.get_one::<PathBuf>("path").unwrap();
    get_dir_child(path);
}

fn get_dir_child(dir:&PathBuf) -> ReadDir {
    let elements = fs::read_dir(dir.as_path()).unwrap();
    elements
}
