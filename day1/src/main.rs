// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    // The first arg is the data file path
    let datafile_arg = match std::env::args().nth(1) {
        Some(path) => path,
        None => {
            eprintln!("Error: no textfile provided.");
            std::process::exit(exitcode::DATAERR);
        }
    };

    let path = Path::new(&datafile_arg);

    println!("Using datafile: {}", path.display());

    // Open the data file
    let file = match File::open(&path) {
        Err(e) => {
            eprintln!("Can't open file. {}", e);
            std::process::exit(exitcode::DATAERR);
        },
        Ok(file) => file,
    };

    // Iterate over the items in the data file converting each line
    // to an int, doing the silly math, and accumulating it
    let mut total = 0;
    let lines = io::BufReader::new(file).lines();
    for line in lines {
        match line {
            Err(_) => (),
            Ok(data) => {
                let data_i = data.parse::<i32>().unwrap();
                let converted_data = (data_i / 3) - 2;
                println!("{} -> {}", data_i, converted_data);
                total = total + converted_data;
            }
        };
    }

    // Print the total
    println!("total: {}", total);
}
