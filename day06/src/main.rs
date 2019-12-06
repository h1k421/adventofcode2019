use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::io::{self, prelude::*, BufReader};

fn get_parents(orbits: &HashMap<String, String>, child: &str, parents: &mut Vec<String>) {
    let parent = orbits.get(child).unwrap();

    if parent != "COM" {
        parents.push(parent.clone());
        get_parents(orbits, parent, parents);
    }
}

fn main() -> io::Result<()> {
    let part = env::args().nth(1).expect("Please a part (1 or 2)");

    let input_path = env::args()
        .nth(2)
        .expect("Please provide a file as argument");
    let reader = BufReader::new(File::open(input_path)?);

    let mut orbits: HashMap<String, String> = HashMap::new();
    reader
        .lines()
        .map(|l| l.unwrap().split(')').map(|s| s.to_string()).collect())
        .for_each(|mut f: Vec<String>| {
            orbits.insert(f.remove(1), f.remove(0));
        });

    match part.as_str() {
        "1" => {
            let mut distances: HashMap<String, usize> = HashMap::new();
            for child in orbits.keys() {
                let mut distance = 0;
                let mut temp_child = child;

                while temp_child != "COM" {
                    distance += 1;
                    temp_child = orbits.get(temp_child).unwrap();
                }

                distances.insert(child.clone(), distance);
            }

            let mut answer = 0;
            for distance in distances.values() {
                answer += distance;
            }

            println!("{:?}", answer);
        }
        "2" => {
            let mut you_parents = Vec::new();
            let mut san_parents = Vec::new();

            get_parents(&orbits, "YOU", &mut you_parents);
            get_parents(&orbits, "SAN", &mut san_parents);

            for (you_steps, you_parent) in you_parents.iter().enumerate() {
                for (san_steps, san_parent) in san_parents.iter().enumerate() {
                    if you_parent == san_parent {
                        println!(
                            "Found intersection at {}, steps: {}",
                            you_parent,
                            you_steps + san_steps
                        );
                        std::process::exit(0);
                    }
                }
            }
        }
        _ => unimplemented!(),
    }

    Ok(())
}
