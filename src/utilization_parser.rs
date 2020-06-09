use log::{debug, error, info};
use std::fs::File;
use std::io::{BufRead, BufReader, Read};
use std::collections::HashMap;
use std::process;
use regex::Regex;

use super::entity_utilization::EntityUtilization;

pub fn parse_file(file: &str) -> HashMap<String, EntityUtilization> {
    let file_a = File::open(file).expect(&format!("Could not open file {}", file));
    let mut reader = BufReader::new(file_a);
    let mut total_lines = 0;

    check_file_header(reader.by_ref(), &mut total_lines);
    seek_utilization_chapter(reader.by_ref(), &mut total_lines);
    parse_utilization_table(reader.by_ref(), &mut total_lines)
}

fn check_file_header(reader: &mut impl BufRead, total_lines: &mut usize) {
    let re_cmd = Regex::new("report_utilization.+-hierarchical").unwrap();
    for line_opt in reader.lines() {
        let line = line_opt.unwrap();
        *total_lines += 1;

        if line.contains("Command") {
            if re_cmd.is_match(&line) {
                info!("Found correct command in file header on line {}.", total_lines);
                return;
            } else {
                error!("The file was created with a wrong command (see line {})!", total_lines);
                process::exit(exitcode::NOINPUT);
            }
        }
        if *total_lines > 10 {
            error!("Could not find a command declaration in input file.");
            process::exit(exitcode::NOINPUT);
        }
    }
}

fn seek_utilization_chapter(reader: &mut impl BufRead, total_lines: &mut usize) {
    for _ in 1..=2 {
        for line_opt in reader.lines() {
            let line = line_opt.unwrap();
            *total_lines += 1;

            if line.contains("Utilization by Hierarchy") {
                info!("Hierarchical utilization chapter starts on line {}.", total_lines);
                return;
            }
        }
    }
}

fn check_columns(hdr: &Vec<&str>, req: &[(usize, &str)]) {
    for r in req {
        if hdr[r.0].to_lowercase().contains(&r.1.to_lowercase()) {
            debug!("Column {} correctly contains {}", r.0, r.1);
        } else {
            error!("Column {} does not seem to contain {}, but: '{}'", r.0, r.1, hdr[r.0]);
        }
    }
}

fn parse_utilization_entries(reader: &mut impl BufRead, total_lines: &mut usize) -> HashMap<String, EntityUtilization> {
    let mut parsed_util = HashMap::new();
    let mut parsed_indent = vec![0];
    let mut parsed_path = Vec::new();
    let mut last_entity = String::new();
    for line_opt in reader.by_ref().lines() {
        let line = line_opt.unwrap();
        *total_lines += 1;

        if line.starts_with("+---------") {
            break;
        }

        let items: Vec<&str> = line.split('|').skip(1).collect();

        let entity_name = String::from(items[0].trim_end());
        let path_indent = entity_name.len() - entity_name.trim().len();
        if &path_indent > parsed_indent.last().unwrap() {
            parsed_path.push(last_entity.trim().to_owned());
            parsed_indent.push(path_indent);
        }
        while &path_indent < parsed_indent.last().unwrap() {
            parsed_path.pop();
            parsed_indent.pop();
        }
        let path = parsed_path.join("/") + "/" + entity_name.trim();

        let entity = EntityUtilization {
            lut: items[2].trim().parse().expect(&format!("Non-numerical value {} for LUT count", items[2])),
            reg: items[6].trim().parse().expect(&format!("Non-numerical value {} for REG count", items[6])),
            dsp: items[10].trim().parse().expect(&format!("Non-numerical value {} for DSP count", items[10])),
            uram: items[9].trim().parse().expect(&format!("Non-numerical value {} for BRAM count", items[9])),
            bram18: items[8].trim().parse::<i32>().expect(&format!("Non-numerical value {} for BRAM count", items[8]))
                + items[7].trim().parse::<i32>().expect(&format!("Non-numerical value {} for BRAM36 count", items[7])) * 2,
        };
        parsed_util.insert(String::from(path), entity);

        last_entity = entity_name;
    }

    parsed_util
}

fn parse_utilization_table(reader: &mut impl BufRead, total_lines: &mut usize) -> HashMap<String, EntityUtilization> {
    for line_opt in reader.lines() {
        let line = line_opt.unwrap();
        *total_lines += 1;

        if line.starts_with("+-----------") {
            break;
        }
    }

    let header_line = reader.lines().next().expect("Early end of file: No header line read.").unwrap();
    let header_items: Vec<&str> = header_line.split('|').skip(1).collect();
    check_columns(&header_items, &[(0, "instance"), (2, "LUT"), (6, "FF"), (7, "RAMB36"), (8, "RAMB18"), (9, "URAM"), (10, "DSP48")]);
    reader.by_ref().lines().next();

    parse_utilization_entries(reader, total_lines)
}
