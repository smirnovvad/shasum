extern crate clap;
extern crate sha1;
extern crate sha2;
use clap::{App, Arg};
use sha2::Digest;
use std::fs;
use std::fs::read_dir;
use std::io::prelude::*;
use std::path::Path;

fn calculate_hash(data: &mut Vec<u8>, sha: usize) -> String {
    let strout: String = match sha {
        224 => format!("{:x}", sha2::Sha224::digest(&data)),
        256 => format!("{:x}", sha2::Sha256::digest(&data)),
        384 => format!("{:x}", sha2::Sha384::digest(&data)),
        512 => format!("{:x}", sha2::Sha512::digest(&data)),
        512224 => format!("{:x}", sha2::Sha512Trunc224::digest(&data)),
        512256 => format!("{:x}", sha2::Sha512Trunc256::digest(&data)),
        _ => format!("{:x}", sha1::Sha1::digest(&data)),
    };
    strout
}

fn read_data(path: &Path, sha: usize) -> String {
    let mut results = Vec::new();
    if path.is_dir() {
        let paths = read_dir(path).unwrap();
        for entry in paths {
            if let Ok(entry) = entry {
                if entry.path().is_file() {
                    if let Ok(mut file) = fs::File::open(&entry.path()) {
                        let mut data = Vec::new();
                        file.read_to_end(&mut data).unwrap();
                        let strout = calculate_hash(&mut data, sha);
                        results.push(strout + "    " + entry.path().to_str().unwrap())
                    }
                }
            }
        }
    } else {
        if let Ok(mut file) = fs::File::open(&path) {
            let mut data = Vec::new();
            file.read_to_end(&mut data).unwrap();
            let strout = calculate_hash(&mut data, sha);
            results.push(strout + "    " + path.to_str().unwrap())
        }
    }
    return results.join("\n");
}

fn main() {
    let matches = App::new("shasum")
        .version("0.5.2")
        .author("Smirnov V. <smirnovvad7@gmail.com>")
        .about("Calculate SHA checksums for input file or directory.")
        .arg(
            Arg::with_name("INPUT FILE")
                .help("Sets the input file to use")
                .required(true)
                .index(1),
        )
        .arg(
            Arg::with_name("algorithm")
                .short("a")
                .takes_value(true)
                .possible_values(&["1", "224", "256", "384", "512", "512224", "512256"])
                .default_value("1"),
        )
        .get_matches();

    // Calling .unwrap() is safe here because "INPUT" is required (if "INPUT" wasn't
    // required we could have used an 'if let' to conditionally get the value)
    let path = Path::new(matches.value_of("INPUT FILE").unwrap());
    match matches.value_of("algorithm").unwrap() {
        "1" => println!("{}", read_data(path, 1)),
        "224" => println!("{}", read_data(path, 224)),
        "256" => println!("{}", read_data(path, 256)),
        "384" => println!("{}", read_data(path, 384)),
        "512" => println!("{}", read_data(path, 512)),
        "512224" => println!("{}", read_data(path, 512224)),
        "512256" => println!("{}", read_data(path, 512256)),
        _ => unreachable!(),
    }
}
