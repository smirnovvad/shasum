extern crate clap;
extern crate crypto;
use self::crypto::digest::Digest;
use self::crypto::sha1::*;
use self::crypto::sha2::*;
use clap::{App, Arg};
use std::error::Error;
use std::fs::read_dir;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

fn calculate_sha(path: &Path, sha: usize) -> String {
    let mut results = Vec::new();
    if path.is_dir() {
        let paths = read_dir(path).unwrap();
        for entry in paths {
            if let Ok(entry) = entry {
                // Here, `entry` is a `DirEntry`.

                let mut file = match File::open(entry.path()) {
                    Err(why) => panic!("Couldn't read {}: {}", path.display(), why.description()),
                    Ok(file) => file,
                };

                let mut data = Vec::new();
                if entry.path().is_file() {
                    match file.read_to_end(&mut data) {
                        Err(why) => panic!(
                            "Couldn't read {}: {}, {:?}",
                            path.display(),
                            why.description(),
                            entry.path()
                        ),
                        Ok(_) => {
                            let mut strout = match sha {
                                224 => {
                                    let mut hasher = Sha224::new();
                                    hasher.input(&data);
                                    hasher.result_str()
                                }
                                256 => {
                                    let mut hasher = Sha256::new();
                                    hasher.input(&data);
                                    hasher.result_str()
                                }
                                384 => {
                                    let mut hasher = Sha384::new();
                                    hasher.input(&data);
                                    hasher.result_str()
                                }
                                512 => {
                                    let mut hasher = Sha512::new();
                                    hasher.input(&data);
                                    hasher.result_str()
                                }
                                512224 => {
                                    let mut hasher = Sha512Trunc224::new();
                                    hasher.input(&data);
                                    hasher.result_str()
                                }
                                512256 => {
                                    let mut hasher = Sha512Trunc256::new();
                                    hasher.input(&data);
                                    hasher.result_str()
                                }
                                _ => {
                                    let mut hasher = Sha1::new();
                                    hasher.input(&data);
                                    hasher.result_str()
                                }
                            };

                            results.push(strout + "    " + entry.path().to_str().unwrap())
                        }
                    }
                }
            }
        }
        results.join("\n")
    } else {
        let mut file = match File::open(&path) {
            Err(why) => panic!("Couldn't read {}: {}", path.display(), why.description()),
            Ok(file) => file,
        };

        let mut data = Vec::new();

        match file.read_to_end(&mut data) {
            Err(why) => panic!("Couldn't read {}: {}", path.display(), why.description()),
            Ok(_) => {
                let mut strout = match sha {
                    224 => {
                        let mut hasher = Sha224::new();
                        hasher.input(&data);
                        hasher.result_str()
                    }
                    256 => {
                        let mut hasher = Sha256::new();
                        hasher.input(&data);
                        hasher.result_str()
                    }
                    384 => {
                        let mut hasher = Sha384::new();
                        hasher.input(&data);
                        hasher.result_str()
                    }
                    512 => {
                        let mut hasher = Sha512::new();
                        hasher.input(&data);
                        hasher.result_str()
                    }
                    512224 => {
                        let mut hasher = Sha512Trunc224::new();
                        hasher.input(&data);
                        hasher.result_str()
                    }
                    512256 => {
                        let mut hasher = Sha512Trunc256::new();
                        hasher.input(&data);
                        hasher.result_str()
                    }
                    _ => {
                        let mut hasher = Sha1::new();
                        hasher.input(&data);
                        hasher.result_str()
                    }
                };
                strout + "    " + path.to_str().unwrap()
            }
        }
    }
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
        "1" => println!("{}", calculate_sha(path, 1)),
        "224" => println!("{}", calculate_sha(path, 224)),
        "256" => println!("{}", calculate_sha(path, 256)),
        "384" => println!("{}", calculate_sha(path, 384)),
        "512" => println!("{}", calculate_sha(path, 512)),
        "512224" => println!("{}", calculate_sha(path, 512224)),
        "512256" => println!("{}", calculate_sha(path, 512256)),
        _ => unreachable!(),
    }
}
