use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn compute_fuel(mass: u64) -> u64 {
    let fuel = (mass as f64 / 3.0).floor() - 2.0;

    if fuel < 0.0 {
        0
    } else {
        fuel as u64
    }
}

fn compute_fuel_part1(mass: u64) -> u64 {
    compute_fuel(mass)
}

fn compute_fuel_part2(mass: u64) -> u64 {
    let fuel = compute_fuel(mass);

    let mut fuel_needed_by_fuel = 0;

    let mut last_fuel = fuel;

    while last_fuel != 0 {
        let tmp = compute_fuel(last_fuel);

        fuel_needed_by_fuel += tmp;

        last_fuel = tmp;
    }

    fuel + fuel_needed_by_fuel
}

fn handle_part1(input: &str) -> std::io::Result<()> {
    let reader = BufReader::new(File::open(input)?);

    let mut result = 0;

    for line in reader.lines() {
        let mass = u64::from_str_radix(&line?, 10).expect("Cannot parse a line as a valid u64");
        result += compute_fuel_part1(mass);
    }

    println!("Total fuel needed: {}", result);
    Ok(())
}

fn handle_part2(input: &str) -> std::io::Result<()> {
    let reader = BufReader::new(File::open(input)?);

    let mut result = 0;

    for line in reader.lines() {
        let mass = u64::from_str_radix(&line?, 10).expect("Cannot parse a line as a valid u64");
        result += compute_fuel_part2(mass);
    }

    println!("Total fuel needed: {}", result);
    Ok(())
}

fn main() -> std::io::Result<()> {
    let part = env::args().nth(1).expect("Please a part (1 or 2)");
    let input_path = env::args()
        .nth(2)
        .expect("Please provide a file as argument");

    match part.as_str() {
        "1" => handle_part1(&input_path),
        "2" => handle_part2(&input_path),
        _ => unimplemented!(),
    }
}

#[cfg(test)]
mod test {
    #[test]
    fn test_part1() {
        use super::compute_fuel_part1;

        assert_eq!(compute_fuel_part1(12), 2);
        assert_eq!(compute_fuel_part1(14), 2);
        assert_eq!(compute_fuel_part1(1969), 654);
        assert_eq!(compute_fuel_part1(100756), 33583);
    }

    #[test]
    fn test_part2() {
        use super::compute_fuel_part2;

        assert_eq!(compute_fuel_part2(12), 2);
        assert_eq!(compute_fuel_part2(1969), 966);
        assert_eq!(compute_fuel_part2(100756), 50346);
    }
}
