use std::env;
use std::process;

use std::vec::Vec;

mod utilization_parser;
mod entity_utilization;

use utilization_parser::parse_file;

fn main() {
    pretty_env_logger::init();

    let args: Vec<_> = env::args().collect();
    if args.len() != 3 {
        println!("Usage: {} file_a file_b", args[0]);
        process::exit(exitcode::USAGE);
    }

    let map_a = parse_file(&args[1]);
    let map_b = parse_file(&args[2]);

    let mut keys : Vec<_> = map_a.keys().collect();
    keys.extend(map_b.keys().collect::<Vec<_>>());
    keys.sort_unstable();
    keys.dedup();

    let mut differences = Vec::new();
    for key in keys {
        let difference = match (map_a.get(key), map_b.get(key)) {
            (Some(a), Some(b)) => b - a,
            (Some(a), _) => -a,
            (_, Some(b)) => b.clone(),
            _ => panic!("Got unexpected key types when building differences.")
        };

        differences.push((difference, key));
    }

    differences.sort_unstable_by_key(|a| (a.1.matches('/').count(), &a.1[..a.1.rfind('/').unwrap()], a.0.lut.abs()));
    for diff in differences {
        println!("LUT: {:6}, REG: {:6}, DSP: {:6}, BRAM18: {:6}, URAM: {:6}   {}", diff.0.lut, diff.0.reg, diff.0.dsp, diff.0.bram18, diff.0.uram, diff.1);
    }

    process::exit(exitcode::OK);
}
