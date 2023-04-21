use std::env;
use std::fs;
use std::str::FromStr;

#[derive(Debug)]
enum Unit {
    Byte,
    Kilobyte,
    Megabyte,
    Gigabyte,
}

impl Unit {
    fn get_factor(unit_result: &Result<Self, String>) -> u64 {
        if let Ok(unit) = unit_result {
            return match unit {
                Unit::Byte => 1,
                Unit::Kilobyte => 1024,
                Unit::Megabyte => 1048576,
                Unit::Gigabyte => 1073741824,
            };
        }

        1
    }
}

impl FromStr for Unit {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "byte" => Ok(Unit::Byte),
            "kilo" => Ok(Unit::Kilobyte),
            "mega" => Ok(Unit::Megabyte),
            "giga" => Ok(Unit::Gigabyte),
            _ => Err(format!("Invalid unit: {}", s)),
        }
    }
}

fn calculate_dir_total(path: &String, factor: &u64) -> std::io::Result<u64> {
    let dir = fs::read_dir(path)?;

    let mut total: u64 = 0;

    for entry_result in dir {
        let entry = entry_result?;

        if entry.file_name().to_string_lossy().starts_with(".") {
            continue;
        }

        let entry_metadata = fs::metadata(entry.path())?;
        let entry_size = entry_metadata.len();

        total += entry_size;
    }

    let result = total / factor;
    Ok(result)
}

fn get_unit(args: &Vec<String>) -> Result<Unit, String> {
    if args.len() != 3 {
        return Ok(Unit::Byte);
    }

    if let Some(unit_arg) = args.get(2) {
        return Unit::from_str(unit_arg);
    }

    Ok(Unit::Byte)
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("Missing arguments");
        std::process::exit(1);
    }

    let unit = get_unit(&args);
    let factor = Unit::get_factor(&unit);

    if let Some(path) = args.get(1) {
        match calculate_dir_total(path, &factor) {
            Ok(total_size) => println!("Total size: {}", total_size),
            Err(error) => eprintln!("Error: {}", error),
        }
    }
}
