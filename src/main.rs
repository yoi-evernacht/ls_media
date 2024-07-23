use std::path::PathBuf;

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

    let path = matches.get_one::<PathBuf>("path").unwrap();
    let pathstr = path.to_str().unwrap();
    if pathstr.starts_with('~') {
        tilda_to_homedir(pathstr);
    }
}

fn tilda_to_homedir(pathstr: &str) -> PathBuf {
    let a = pathstr.chars().skip(1).collect::<String>();
    let path = dirs::home_dir().unwrap().join(a);
    path
}
