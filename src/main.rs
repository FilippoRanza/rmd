extern crate clap;

use rmd::{remove, remove_duplicates_files, Mode, remove_old_files, remove_new_files};

use clap::{App, Arg, ArgMatches};

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

    let parser = parser.arg(
        Arg::with_name("files")
            .multiple(true)
            .help("remove files")
    );

    parser.get_matches()
}

fn get_mode(force: bool, interactive: bool) -> Mode {
    if force {
        Mode::Force
    } else if interactive {
        Mode::Interactive
    } else {
        Mode::Standard
    }
}



fn run_remove<'a>(args: ArgMatches<'a>) -> Result<(), std::io::Error> {
    let recursive = args.is_present("recursive");
    let interactive = args.is_present("interactive");
    let force = args.is_present("force");
    let mode = get_mode(force, interactive);

    let files = match args.values_of("files") {
        Some(file_args) => file_args.collect(),
        None => vec!["."]
    };


    if args.is_present("newer") {
        let time_spec = args.value_of("newer").unwrap();
        remove_new_files(&files, time_spec, mode)?; 
    } else if args.is_present("duplicates") {
        remove_duplicates_files(&files, mode)?;
    } else if args.is_present("older") {
        let time_spec = args.value_of("older").unwrap();
        remove_old_files(&files, time_spec, mode)?;
    } else if args.is_present("files") {
        remove(&files, recursive, mode)?;
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
