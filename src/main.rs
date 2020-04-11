extern crate clap;

use clap::{App, Arg, ArgGroup, ArgMatches};
use rmd::engine;
use rmd::logger;

pub fn parse_args<'a>() -> ArgMatches<'a> {
    let parser = App::new("rmd")
        .about("rm able to remove duplicate files")
        .version("0.4.1")
        .author("Filippo Ranza");

    let parser = parser.arg(
        Arg::with_name("force")
            .short("-f")
            .long("--force")
            .help("ignore nonexistent files and arguments, never prompt")
            .conflicts_with("interactive"),
    );

    let parser = parser.arg(
        Arg::with_name("clean")
            .short("-c")
            .long("--clean")
            .help("remove directories left empty after an automatic removal"),
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
            .help("recursevely remove duplicated file(keep one copy)"),
    );

    let parser = parser.arg(
        Arg::with_name("older")
            .long("--older")
            .help("remove file older then the given time specification")
            .takes_value(true),
    );

    let parser = parser.arg(
        Arg::with_name("newer")
            .long("--newer")
            .help("remove file newer then the given time specification")
            .takes_value(true),
    );

    let parser = parser.arg(
        Arg::with_name("smaller")
            .long("--smaller")
            .help("remove file smaller then the given size specification")
            .takes_value(true),
    );

    let parser = parser.arg(
        Arg::with_name("larger")
            .long("--larger")
            .help("remove file larger then the given size specification")
            .takes_value(true),
    );

    let parser = parser.arg(
        Arg::with_name("verbose")
            .short("-v")
            .long("--verbose")
            .multiple(true)
            .help("print remove status, use multiple twice for a more verbose output"),
    );

    let parser = parser.arg(
        Arg::with_name("log")
            .short("-l")
            .long("--log")
            .multiple(true)
            .help("send status messages to syslog"),
    );

    let parser = parser.group(ArgGroup::with_name("automatic removal").args(&[
        "older",
        "newer",
        "smaller",
        "larger",
        "duplicates",
    ]));

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

fn build_logger<'a>(args: &'a ArgMatches<'a>) -> Option<logger::StatusLogger> {

    let mut status_logger = logger::StatusLogger::new();
    if args.is_present("verbose") {
        let level = logger::get_levle_from_int(args.occurrences_of("verbose"));
        status_logger.add_verbose(level);
    }
    
    if args.is_present("log") {
        let level = logger::get_levle_from_int(args.occurrences_of("log"));
        status_logger.add_logger(level);
    }

    if status_logger.is_used() {
        Some(status_logger)
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
    let mut log = build_logger(&args);
    if let Some(command) = command {
        let clean = args.is_present("clean");
        engine::automatic_remove(&files, mode, command, clean, &mut log)?;
    } else if arg_set {
        engine::remove(&files, mode, args.is_present("recursive"), &mut log)?;
    }

    if let Some(mut log) = log {
        log.log_statistics();
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
