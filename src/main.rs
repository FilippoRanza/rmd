
extern crate clap;

use rmd::{remove, remove_duplicates, Mode};

use clap::{App, Arg, ArgMatches};

pub fn parse_args<'a>() -> ArgMatches<'a> {
    let parser = App::new("rmd")
                        .about("rm able to remove duplicate files")
                        .version("0.2.0")
                        .author("Filippo Ranza");

    let parser = parser.arg(
        Arg::with_name("force").short("-f").long("--force").help("ignore nonexistent files and arguments, never prompt").conflicts_with("interactive")
    );

    let parser = parser.arg(
        Arg::with_name("interactive").short("-i").long("--inter").help("prompt before every removal(not implemented)").conflicts_with("force")
    );

    let parser = parser.arg(
        Arg::with_name("duplicates").short("-d").long("--duplicates").help("recursevely remove duplicated file(keep one copy)").takes_value(true).default_value(".").multiple(true)
    );

    let parser = parser.arg(
        Arg::with_name("recursive").short("-r").long("--recursive").help("remove directories and their contents recursively")
    );

    let parser = parser.arg(
        Arg::with_name("files").multiple(true).help("remove files").conflicts_with("duplicates")
    );
            
    parser.get_matches()
}


fn get_mode(force: bool, interactive: bool) -> Mode {
    if force {
        Mode::Force
    }
    else if interactive {
        Mode::Interactive
    }
    else {
        Mode::Standard
    }
}

fn run_remove<'a>(args: ArgMatches<'a>) -> Result<(), std::io::Error> {        
    let recursive = args.is_present("recursive");
    let interactive = args.is_present("interactive");
    let force = args.is_present("force");
    let mode = get_mode(force, interactive);
    if args.is_present("files") {
        let files: Vec<&str> = args.values_of("files").unwrap().collect();
        remove(&files, recursive, mode)?;
    }
    else {
        let files: Vec<&str> = args.values_of("duplicates").unwrap().collect();
        remove_duplicates(&files, mode)?;
    }
    Ok(())
}


fn main() {
    let args  = parse_args();
    match run_remove(args) {
        Ok(()) => {}
        Err(error) => eprintln!("{}", error)
    };
    
}
