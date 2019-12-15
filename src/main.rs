extern crate clap;

use clap::{App, Arg, ArgMatches};

fn parse_args<'a>() -> ArgMatches<'a> {
    let parser = App::new("rmd")
                        .about("rm able to remove duplicate files")
                        .version("0.2.0")
                        .author("Filippo Ranza");

    let parser = parser.arg(
        Arg::with_name("force").short("-f").long("--force").help("ignore nonexistent files and arguments, never prompt")
    );

    let parser = parser.arg(
        Arg::with_name("interactive").short("-i").long("--inter").help("ignore nonexistent files and arguments, never prompt")
    );

    let parser = parser.arg(
        Arg::with_name("duplicates").short("-d").long("--duplicates").help("recursevely remove duplicated file(keep one copy)")
    );

    let parser = parser.arg(
        Arg::with_name("recursive").short("-r").long("--recursive").help("remove directories and their contents recursively")
    );

    let parser = parser.arg(
        Arg::with_name("files").multiple(true).help("remove files")
    );
            
    parser.get_matches()
}

fn main() {
    let  args  = parse_args();
    println!("{:?}", args);
}
