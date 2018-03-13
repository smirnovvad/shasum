extern crate clap;
extern crate crypto;
use clap::{Arg, App};
use std::error::Error;
use std::fs::read_dir;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use self::crypto::digest::Digest;
use self::crypto::sha1::*;
use self::crypto::sha2::*;


fn calculate_sha512256(path: &Path) -> String {
    let mut results = Vec::new();
    if path.is_dir() {

            let paths = read_dir(path).unwrap();
            for entry in paths {
                if let Ok(entry) = entry {
                    // Here, `entry` is a `DirEntry`.

                    let mut file = match File::open(entry.path()) {
                        Err(why) => {
                            panic!("Couldn't read {}: {}", path.display(), why.description())
                        }
                        Ok(file) => file,
                    };

                    let mut data = Vec::new();
                    if entry.path().is_file() {
                        match file.read_to_end(&mut data) {
                            Err(why) => {
                                panic!(
                                    "Couldn't read {}: {}, {:?}",
                                    path.display(),
                                    why.description(),
                                    entry.path()
                                )
                            }
                            Ok(_) => {
                                let mut hasher = Sha512Trunc256::new();
                                hasher.input(&data);
                                results.push(
                                    hasher.result_str() + "    " + entry.path().to_str().unwrap(),
                                )
                            }
                        }
                    }
                }
            }
            results.join("\n")

        }

        else {
            let mut file = match File::open(&path) {
                Err(why) => panic!("Couldn't read {}: {}", path.display(), why.description()),
                Ok(file) => file,
            };

            let mut data = Vec::new();

            match file.read_to_end(&mut data) {
                Err(why) => panic!("Couldn't read {}: {}", path.display(), why.description()),
                Ok(_) => {
                    let mut hasher = Sha512Trunc256::new();
                    hasher.input(&data);
                    hasher.result_str() + "    " + path.to_str().unwrap()
                }
            }
        }
    }



fn calculate_sha512224(path: &Path) -> String {

    let mut results = Vec::new();
    if path.is_dir() {

            let paths = read_dir(path).unwrap();
            for entry in paths {
                if let Ok(entry) = entry {
                    // Here, `entry` is a `DirEntry`.

                    let mut file = match File::open(entry.path()) {
                        Err(why) => {
                            panic!("Couldn't read {}: {}", path.display(), why.description())
                        }
                        Ok(file) => file,
                    };

                    let mut data = Vec::new();
                    if entry.path().is_file() {
                        match file.read_to_end(&mut data) {
                            Err(why) => {
                                panic!(
                                    "Couldn't read {}: {}, {:?}",
                                    path.display(),
                                    why.description(),
                                    entry.path()
                                )
                            }
                            Ok(_) => {
                                let mut hasher = Sha512Trunc224::new();
                                hasher.input(&data);
                                results.push(
                                    hasher.result_str() + "    " + entry.path().to_str().unwrap(),
                                )
                            }
                        }
                    }
                }
            }
            results.join("\n")

        }

        else {
            let mut file = match File::open(&path) {
                Err(why) => panic!("Couldn't read {}: {}", path.display(), why.description()),
                Ok(file) => file,
            };

            let mut data = Vec::new();

            match file.read_to_end(&mut data) {
                Err(why) => panic!("Couldn't read {}: {}", path.display(), why.description()),
                Ok(_) => {
                    let mut hasher = Sha512Trunc224::new();
                    hasher.input(&data);
                    hasher.result_str() + "    " + path.to_str().unwrap()
                }
            }
        }
    }



fn calculate_sha512(path: &Path) -> String {
    let mut results = Vec::new();
    if path.is_dir() {

            let paths = read_dir(path).unwrap();
            for entry in paths {
                if let Ok(entry) = entry {
                    // Here, `entry` is a `DirEntry`.

                    let mut file = match File::open(entry.path()) {
                        Err(why) => {
                            panic!("Couldn't read {}: {}", path.display(), why.description())
                        }
                        Ok(file) => file,
                    };

                    let mut data = Vec::new();
                    if entry.path().is_file() {
                        match file.read_to_end(&mut data) {
                            Err(why) => {
                                panic!(
                                    "Couldn't read {}: {}, {:?}",
                                    path.display(),
                                    why.description(),
                                    entry.path()
                                )
                            }
                            Ok(_) => {
                                let mut hasher = Sha512::new();
                                hasher.input(&data);
                                results.push(
                                    hasher.result_str() + "    " + entry.path().to_str().unwrap(),
                                )
                            }
                        }
                    }
                }
            }
            results.join("\n")

        }

        else {
            let mut file = match File::open(&path) {
                Err(why) => panic!("Couldn't read {}: {}", path.display(), why.description()),
                Ok(file) => file,
            };

            let mut data = Vec::new();

            match file.read_to_end(&mut data) {
                Err(why) => panic!("Couldn't read {}: {}", path.display(), why.description()),
                Ok(_) => {
                    let mut hasher = Sha512::new();
                    hasher.input(&data);
                    hasher.result_str() + "    " + path.to_str().unwrap()
                }
            }
        }
    }




fn calculate_sha384(path: &Path) -> String {
    let mut results = Vec::new();
    if path.is_dir() {

            let paths = read_dir(path).unwrap();
            for entry in paths {
                if let Ok(entry) = entry {
                    // Here, `entry` is a `DirEntry`.

                    let mut file = match File::open(entry.path()) {
                        Err(why) => {
                            panic!("Couldn't read {}: {}", path.display(), why.description())
                        }
                        Ok(file) => file,
                    };

                    let mut data = Vec::new();
                    if entry.path().is_file() {
                        match file.read_to_end(&mut data) {
                            Err(why) => {
                                panic!(
                                    "Couldn't read {}: {}, {:?}",
                                    path.display(),
                                    why.description(),
                                    entry.path()
                                )
                            }
                            Ok(_) => {
                                let mut hasher = Sha384::new();
                                hasher.input(&data);
                                results.push(
                                    hasher.result_str() + "    " + entry.path().to_str().unwrap(),
                                )
                            }
                        }
                    }
                }
            }
            results.join("\n")

        }

        else {
            let mut file = match File::open(&path) {
                Err(why) => panic!("Couldn't read {}: {}", path.display(), why.description()),
                Ok(file) => file,
            };

            let mut data = Vec::new();

            match file.read_to_end(&mut data) {
                Err(why) => panic!("Couldn't read {}: {}", path.display(), why.description()),
                Ok(_) => {
                    let mut hasher = Sha384::new();
                    hasher.input(&data);
                    hasher.result_str() + "    " + path.to_str().unwrap()
                }
            }
        }
    }


fn calculate_sha256(path: &Path) -> String {

    let mut results = Vec::new();
    if path.is_dir() {

            let paths = read_dir(path).unwrap();
            for entry in paths {
                if let Ok(entry) = entry {
                    // Here, `entry` is a `DirEntry`.

                    let mut file = match File::open(entry.path()) {
                        Err(why) => {
                            panic!("Couldn't read {}: {}", path.display(), why.description())
                        }
                        Ok(file) => file,
                    };

                    let mut data = Vec::new();
                    if entry.path().is_file() {
                        match file.read_to_end(&mut data) {
                            Err(why) => {
                                panic!(
                                    "Couldn't read {}: {}, {:?}",
                                    path.display(),
                                    why.description(),
                                    entry.path()
                                )
                            }
                            Ok(_) => {
                                let mut hasher = Sha256::new();
                                hasher.input(&data);
                                results.push(
                                    hasher.result_str() + "    " + entry.path().to_str().unwrap(),
                                )
                            }
                        }
                    }
                }
            }
            results.join("\n")

        }

        else {
            let mut file = match File::open(&path) {
                Err(why) => panic!("Couldn't read {}: {}", path.display(), why.description()),
                Ok(file) => file,
            };

            let mut data = Vec::new();

            match file.read_to_end(&mut data) {
                Err(why) => panic!("Couldn't read {}: {}", path.display(), why.description()),
                Ok(_) => {
                    let mut hasher = Sha256::new();
                    hasher.input(&data);
                    hasher.result_str() + "    " + path.to_str().unwrap()
                }
            }
        }
    }



fn calculate_sha224(path: &Path) -> String {

    let mut results = Vec::new();
    if path.is_dir() {

            let paths = read_dir(path).unwrap();
            for entry in paths {
                if let Ok(entry) = entry {
                    // Here, `entry` is a `DirEntry`.

                    let mut file = match File::open(entry.path()) {
                        Err(why) => {
                            panic!("Couldn't read {}: {}", path.display(), why.description())
                        }
                        Ok(file) => file,
                    };

                    let mut data = Vec::new();
                    if entry.path().is_file() {
                        match file.read_to_end(&mut data) {
                            Err(why) => {
                                panic!(
                                    "Couldn't read {}: {}, {:?}",
                                    path.display(),
                                    why.description(),
                                    entry.path()
                                )
                            }
                            Ok(_) => {
                                let mut hasher = Sha224::new();
                                hasher.input(&data);
                                results.push(
                                    hasher.result_str() + "    " + entry.path().to_str().unwrap(),
                                )
                            }
                        }
                    }
                }
            }
            results.join("\n")

        }

        else {
            let mut file = match File::open(&path) {
                Err(why) => panic!("Couldn't read {}: {}", path.display(), why.description()),
                Ok(file) => file,
            };

            let mut data = Vec::new();

            match file.read_to_end(&mut data) {
                Err(why) => panic!("Couldn't read {}: {}", path.display(), why.description()),
                Ok(_) => {
                    let mut hasher = Sha224::new();
                    hasher.input(&data);
                    hasher.result_str() + "    " + path.to_str().unwrap()
                }
            }
        }

}


fn calculate_sha1(path: &Path) -> String {

    let mut results = Vec::new();
    if path.is_dir() {
            let paths = read_dir(path).unwrap();
            for entry in paths {
                if let Ok(entry) = entry {
                    // Here, `entry` is a `DirEntry`.

                    let mut file = match File::open(entry.path()) {
                        Err(why) => {
                            panic!("Couldn't read {}: {}", path.display(), why.description())
                        }
                        Ok(file) => file,
                    };

                    let mut data = Vec::new();
                    if entry.path().is_file() {
                        match file.read_to_end(&mut data) {
                            Err(why) => {
                                panic!(
                                    "Couldn't read {}: {}, {:?}",
                                    path.display(),
                                    why.description(),
                                    entry.path()
                                )
                            }
                            Ok(_) => {
                                let mut hasher = Sha1::new();
                                hasher.input(&data);
                                results.push(
                                    hasher.result_str() + "    " + entry.path().to_str().unwrap(),
                                )
                            }
                        }
                    }

                }
            }
            results.join("\n")

        }

        else {
            let mut file = match File::open(&path) {
                Err(why) => panic!("Couldn't read {}: {}", path.display(), why.description()),
                Ok(file) => file,
            };

            let mut data = Vec::new();

            match file.read_to_end(&mut data) {
                Err(why) => panic!("Couldn't read {}: {}", path.display(), why.description()),
                Ok(_) => {
                    //let mut result = vec![0; Sha1::output_bytes()];
                    //let mut sha = Sha1::default();
                    let mut hasher = Sha1::new();
                    hasher.input(&data);
                    hasher.result_str() + "    " + path.to_str().unwrap()
                }
            }
        }
    }



fn main() {
    let matches = App::new("shasum")
        .version("0.5.0")
        .author("Smirnov V. <smirnovvad7@gmail.com>")
        .about("Print SHA checksums.")
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
        "1" => println!("{}", calculate_sha1(path)),
        "224" => println!("{}", calculate_sha224(path)),
        "256" => println!("{}", calculate_sha256(path)),
        "384" => println!("{}", calculate_sha384(path)),
        "512" => println!("{}", calculate_sha512(path)),
        "512224" => println!("{}", calculate_sha512224(path)),
        "512256" => println!("{}", calculate_sha512256(path)),
        _ => unreachable!(),
    }
}
