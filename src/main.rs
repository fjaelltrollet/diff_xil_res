use std::env;

mod utilization_parser;
use utilization_parser::{check_file_header, seek_utilization_chapter, parse_utilization_table};

fn main() {
    pretty_env_logger::init();

    let args: Vec<_> = env::args().collect();
    if args.len() != 3 {
        println!("Usage: {} file_a file_b", args[0]);
        process::exit(exitcode::USAGE);
    }


    process::exit(exitcode::OK);
}
