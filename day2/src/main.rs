// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.
//
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

/// Run the given program and return the output
fn run_program(data : &Vec<String>) -> Vec<String> {
    let mut data_int = Vec::with_capacity(data.len());
    data_int.extend(data.iter().map(|val| {
        val.parse::<i32>().unwrap()
    }));

    let end_i : usize = data_int.len();
    let mut i : usize = 0;
    loop {
        if i >= end_i {
            break;
        }

        let opcode = data_int[i];
        match opcode {
            1 => {
                let lhs_i = data_int[i+1] as usize;
                let rhs_i = data_int[i+2] as usize;
                let dest_i = data_int[i+3] as usize;
                data_int[dest_i] = data_int[lhs_i] + data_int[rhs_i];
                i = i + 4;
            },
            2 => {
                let lhs_i = data_int[i+1] as usize;
                let rhs_i = data_int[i+2] as usize;
                let dest_i = data_int[i+3] as usize;
                data_int[dest_i] = data_int[lhs_i] * data_int[rhs_i];
                i = i + 4;
            },
            99 => {
                break;
            },
            _ => {
                eprintln!("Unknown operand: {}", data_int[i]);
                std::process::exit(exitcode::DATAERR);
            },
        }
    }

    let mut data_out : Vec<String> = Vec::with_capacity(data.len());
    for item in &data_int {
        data_out.push(item.to_string());
    }
    data_out
}

#[cfg(test)]
mod tests {
    use super::*;

    fn stringify(data : Vec<&str>) -> Vec<String> {
        data.iter().map(ToString::to_string).collect()
    }

    /// Test 1 oper
    #[test]
    fn test_run_program_oper_1() {
        assert_eq!(
            run_program(stringify(vec!["1", "0", "0", "0", "99"])),
            stringify(vec!["2", "0", "0", "0", "99"])
        );
    }

    /// Test 2 oper
    #[test]
    fn test_run_program_oper_2() {
        assert_eq!(
            run_program(stringify(vec!["2", "3", "0", "3", "99"])),
            stringify(vec!["2", "3", "0", "6", "99"])
        );
    }

    /// Test complex
    #[test]
    fn test_run_program_complex() {
        assert_eq!(
            run_program(stringify(vec!["1", "1", "1", "4", "99", "5", "6", "0", "99"])),
            stringify(vec!["30", "1", "1", "4", "2", "5", "6", "0", "99"])
        );
    }
}

fn main() {
    // First arg is the data file path
    let datafile_arg = match std::env::args().nth(1) {
        Some(path) => path,
        None => {
            eprintln!("Error: no textfile provided.");
            std::process::exit(exitcode::DATAERR);
        }
    };

    let mut file = match File::open(Path::new(&datafile_arg)) {
        Err(e) => {
            eprintln!("Can't open file: {}", e);
            std::process::exit(exitcode::DATAERR);
        },
        Ok(file) => file,
    };

    let mut contents = String::new();
    match file.read_to_string(&mut contents) {
        Ok(_) => (),
        Err(e) => {
            eprintln!("Can't read file: {}", e);
            std::process::exit(exitcode::DATAERR);
        },
    }

    // Remove whitespace from beginning and end
    let contents = contents.trim();
    println!("input: {}", contents);

    // Split the input on ","
    let mut prog_input: Vec<String> = contents.split(",").map(ToString::to_string).collect();

    'outer: for input_noun in 0..99 {
        for input_verb in 0..99 {
            // Replace position 1 with 12
            prog_input[1] = input_noun.to_string();

            // Replace position 2 with 2
            prog_input[2] = input_verb.to_string();

            let prog_output = run_program(&prog_input);
            println!("output: {}", prog_output.join(","));
            println!("position 0: {}", prog_output[0]);
            if prog_output[0] == "19690720" {
                println!("eureka!: noun={} verb={}", input_noun, input_verb);
                println!("{}", 100 * input_noun + input_verb);
                break 'outer;
            }
        }
    }
}
