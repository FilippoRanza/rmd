extern crate clap;

use clap::{App, Arg, ArgMatches};
use rmd::engine;

pub fn parse_args<'a>() -> ArgMatches<'a> {
    let parser = App::new("rmd")
        .about("rm able to remove duplicate files")
        .version("0.3.0")
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
            .conflicts_with_all(&["older", "newer", "smaller", "larger"]),
    );

    let parser = parser.arg(
        Arg::with_name("older")
            .long("--older")
            .help("remove file older then the given time specification")
            .takes_value(true)
            .conflicts_with_all(&["duplicates", "newer", "smaller", "larger"]),
    );

    let parser = parser.arg(
        Arg::with_name("newer")
            .long("--newer")
            .help("remove file newer then the given time specification")
            .takes_value(true)
            .conflicts_with_all(&["duplicates", "older",  "smaller", "larger"]),
    );

    let parser = parser.arg(
        Arg::with_name("smaller")
            .long("--smaller")
            .help("remove file smaller then the given size specification")
            .takes_value(true)
            .conflicts_with_all(&["duplicates", "older", "newer", "larger"]),
    );

    let parser = parser.arg(
        Arg::with_name("larger")
            .long("--larger")
            .help("remove file larger then the given size specification")
            .takes_value(true)
            .conflicts_with_all(&["duplicates", "older", "newer", "smaller"]),
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

fn build_command<'a>(args: &'a ArgMatches<'a>) -> Option<engine::Command<'a>> {
    if args.is_present("duplicates") {
        Some(engine::Command::Duplicates)
    } else if args.is_present("older") {
        let time_spec = args.value_of("older").unwrap();
        Some(engine::Command::ByDate((time_spec, true)))
    } else if args.is_present("newer") {
        let time_spec = args.value_of("newer").unwrap();
        Some(engine::Command::ByDate((time_spec, false)))
    } else if args.is_present("smaller") {
        let size_spec = args.value_of("smaller").unwrap();
        Some(engine::Command::BySize((size_spec, true)))
    } else if args.is_present("larger") {
        let size_spec = args.value_of("larger").unwrap();
        Some(engine::Command::BySize((size_spec, false)))
    } else {
        None
    }
}

fn run_remove<'a>(args: ArgMatches<'a>) -> std::io::Result<()> {
    let mode = get_mode(args.is_present("force"), args.is_present("interactive"));
    let (files, arg_set) = match args.values_of("files") {
        Some(file_args) => (file_args.collect(), true),
        None => (vec!["."], false),
    };

    let command = build_command(&args);
    if let Some(command) = command {
        engine::automatic_remove(&files, mode, command)?;
    } else if arg_set {
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
