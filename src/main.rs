extern crate clap;
extern crate sha1;
extern crate sha2;
use clap::{App, Arg};
use sha2::Digest;
use std::fs;
use std::fs::read_dir;
use std::io::{self, Read};
use std::path::Path;

fn calculate_hash(data: &[u8], sha: usize) -> String {
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
                        let strout = calculate_hash(&data, sha);
                        results.push(strout + "    " + entry.path().to_str().unwrap())
                    }
                }
            }
        }
    } else {
        if let Ok(mut file) = fs::File::open(&path) {
            let mut data = Vec::new();
            file.read_to_end(&mut data).unwrap();
            let strout = calculate_hash(&data, sha);
            results.push(strout + "    " + path.to_str().unwrap())
        }
    }
    return results.join("\n");
}

fn main() {
    let matches = App::new("shasum")
        .version("0.7.0")
        .author("Smirnov V. <smirnovvad7@gmail.com>")
        .about("Print SHA checksums from STDIN, input file or directory.")
        .arg(
            Arg::new("FILE")
                .about("With no FILE, or when FILE is -, read standard input.")
                .takes_value(true)
                .required(false)
                .index(1),
        )
        .arg(
            Arg::new("algorithm")
                .short('a')
                .long("algorithm")
                .value_name("algorithm")
                .about("Sets the algorithm to use")
                .takes_value(true)
                .required(false)
                .possible_values(&["1", "224", "256", "384", "512", "512224", "512256"])
                .default_value("1"),
        )
        .get_matches();

    let sha: usize = matches.value_of("algorithm").unwrap().parse().unwrap();

    match matches.value_of("FILE") {
        Some("-") | None => {
            let mut buffer = String::new();
            let stdin = io::stdin();
            let mut handle = stdin.lock();

            handle.read_to_string(&mut buffer).unwrap();
            println!("{}", calculate_hash(&buffer.as_bytes(), sha));
        }
        Some(path) => {
            let path = Path::new(path);
            println!("{}", read_data(path, sha));
        }
    }
}
