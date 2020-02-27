extern crate clap;

use clap::{App, Arg, ArgMatches};
use rmd::engine;

pub fn parse_args<'a>() -> ArgMatches<'a> {
    let parser = App::new("rmd")
        .about("rm able to remove duplicate files")
        .version("0.2.0")
        .author("Filippo Ranza");

    let parser = parser.arg(
        Arg::with_name("force")
            .short("-f")
            .long("--force")
            .help("ignore nonexistent files and arguments, never prompt")
            .conflicts_with("interactive"),
    );

    let parser = parser.arg(
        Arg::with_name("interactive")
            .short("-i")
            .long("--inter")
            .help("prompt before every removal")
            .conflicts_with("force"),
    );

    let parser = parser.arg(
        Arg::with_name("duplicates")
            .short("-d")
            .long("--duplicates")
            .help("recursevely remove duplicated file(keep one copy)")
            .conflicts_with_all(&["older", "newer"]),
    );

    let parser = parser.arg(
        Arg::with_name("older")
            .long("--older")
            .help("remove file older then the given time specification")
            .takes_value(true)
            .conflicts_with_all(&["duplicates", "newer"]),
    );

    let parser = parser.arg(
        Arg::with_name("newer")
            .long("--newer")
            .help("remove file newer then the given time specification")
            .takes_value(true)
            .conflicts_with_all(&["duplicates", "older"]),
    );

    let parser = parser.arg(
        Arg::with_name("recursive")
            .short("-r")
            .long("--recursive")
            .help("remove directories and their contents recursively"),
    );

    let parser = parser.arg(Arg::with_name("files").multiple(true).help("remove files"));

    parser.get_matches()
}

fn get_mode(force: bool, interactive: bool) -> engine::Mode {
    if force {
        engine::Mode::Force
    } else if interactive {
        engine::Mode::Interactive
    } else {
        engine::Mode::Standard
    }
}

fn build_command<'a>(args: &ArgMatches<'a>) -> Option<engine::Command> {
    if args.is_present("duplicates") {
        Some(engine::Command::Duplicates)
    } else if args.is_present("older") {
        let time_spec = args.value_of("older").unwrap();
        Some(engine::Command::ByDate((time_spec.to_owned(), true)))
    } else if args.is_present("newer") {
        let time_spec = args.value_of("newer").unwrap();
        Some(engine::Command::ByDate((time_spec.to_owned(), false)))
    } else {
        None
    }
}

fn run_remove<'a>(args: ArgMatches<'a>) -> std::io::Result<()> {
    let mode = get_mode(args.is_present("force"), args.is_present("interactive"));
    let files = match args.values_of("files") {
        Some(file_args) => file_args.collect(),
        None => vec!["."],
    };

    let command = build_command(&args);
    if let Some(command) = command {
        engine::automatic_remove(&files, mode, command)?;
    } else {
        engine::remove(&files, mode, args.is_present("recursive"))?;
    }

    Ok(())
}

fn main() {
    let args = parse_args();
    match run_remove(args) {
        Ok(()) => {}
        Err(error) => eprintln!("{}", error),
    };
}
