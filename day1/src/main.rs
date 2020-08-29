// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

/// Return the amount of fuel required for this mass plus the fuel required.
fn get_fuel(mass: i32) -> i32 {
    let mut total = 0;
    let mut fuel = (mass / 3) - 2;
    while fuel > 0 {
        total = total + fuel;
        fuel = (fuel / 3) - 2;
    }
    total
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_fuel() {
        assert_eq!(get_fuel(-1), 0);
        assert_eq!(get_fuel(0), 0);
        assert_eq!(get_fuel(12), 2);
        assert_eq!(get_fuel(100), 39);
    }
}

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
                let fuel = get_fuel(data_i);
                println!("{} -> {}", data_i, fuel);
                total = total + fuel;
            }
        };
    }

    // Print the total
    println!("total: {}", total);
}
