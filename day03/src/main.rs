use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Debug, PartialEq, Eq)]
enum Movement {
    Up(i64),
    Down(i64),
    Right(i64),
    Left(i64),
}

impl Movement {
    pub fn from_raw(value: &str) -> Movement {
        let direction = value.chars().next().unwrap();

        let value = i64::from_str_radix(&value[1..], 10).unwrap();

        match direction {
            'U' => Movement::Up(value),
            'D' => Movement::Down(value),
            'R' => Movement::Right(value),
            'L' => Movement::Left(value),
            _ => unimplemented!(),
        }
    }

    pub fn get_position(&self) -> Position {
        match self {
            Movement::Up(value) => Position { y: *value, x: 0 },
            Movement::Down(value) => Position { y: -*value, x: 0 },
            Movement::Right(value) => Position { y: 0, x: *value },
            Movement::Left(value) => Position { y: 0, x: -*value },
        }
    }
}

#[derive(Debug, PartialEq, Eq, Default, Clone)]
struct Position {
    x: i64,
    y: i64,
}

#[derive(Debug, PartialEq, Eq, Default, Clone)]
struct Intersection {
    position: Position,
    steps: i64,
}

impl Position {
    pub fn merge_positions(&mut self, other: &Position) {
        self.x += other.x;
        self.y += other.y;
    }

    pub fn as_points(&self, base_position: &Position) -> Vec<Position> {
        let mut result = Vec::new();
        let mut final_position = base_position.clone();
        final_position.merge_positions(&self);

        let diff_x = final_position.x - base_position.x;
        let modifier_x = if diff_x < 0 { -1 } else { 1 };

        let diff_y = final_position.y - base_position.y;
        let modifier_y = if diff_y < 0 { -1 } else { 1 };

        for x in 0..diff_x.abs() {
            let mut temp_position = base_position.clone();

            temp_position.x += x * modifier_x;

            result.push(temp_position);
        }

        for y in 0..diff_y.abs() {
            let mut temp_position = base_position.clone();

            temp_position.y += y * modifier_y;

            result.push(temp_position);
        }

        result
    }
}

#[derive(Debug)]
struct WireInfo {
    positions: Vec<Position>,
}

impl WireInfo {
    pub fn new(movements: Vec<Movement>) -> Self {
        let mut positions = Vec::new();
        let mut temp_position = Position::default();

        for movement in movements {
            let movement_velocity = movement.get_position();
            let mut movement_points = movement_velocity.as_points(&temp_position);

            positions.append(&mut movement_points);

            temp_position.merge_positions(&movement_velocity);
        }

        WireInfo { positions }
    }

    pub fn get_intersections(&self, other: &WireInfo) -> Vec<Intersection> {
        let mut result = Vec::new();

        for (step, position) in self.positions.iter().enumerate() {
            for (other_step, other_position) in other.positions.iter().enumerate() {
                if *position == *other_position && *position != Position::default() {
                    result.push(Intersection {
                        position: position.clone(),
                        steps: (step + other_step) as i64,
                    });
                }
            }
        }

        result
    }
}

fn parse_wire_infos(input_file: &str) -> std::io::Result<Vec<WireInfo>> {
    let reader = BufReader::new(File::open(input_file)?);

    let mut result = Vec::new();

    for line in reader.lines() {
        let line = line?;

        let raw_movements = line.split(',');
        let mut movements = Vec::new();

        for movement in raw_movements {
            movements.push(Movement::from_raw(movement));
        }

        result.push(WireInfo::new(movements));
    }

    Ok(result)
}

fn get_min_distance(intersections: Vec<Intersection>, fast_mode: bool) -> i64 {
    let mut min_distance = None;

    for intersection in intersections {
        let distance = if fast_mode {
            intersection.steps
        } else {
            intersection.position.x.abs() + intersection.position.y.abs()
        };

        if let Some(min_distance_value) = min_distance {
            min_distance = Some(std::cmp::min(min_distance_value, distance));
        } else {
            min_distance = Some(distance);
        }
    }

    min_distance.unwrap()
}

fn main() -> std::io::Result<()> {
    let part = env::args().nth(1).expect("Please a part (1 or 2)");
    let input_path = env::args()
        .nth(2)
        .expect("Please provide a file as argument");

    let wires = parse_wire_infos(&input_path)?;
    let intersections = wires[0].get_intersections(&wires[1]);

    match part.as_str() {
        "1" => println!("min distance: {:?}", get_min_distance(intersections, false)),
        "2" => println!("min distance: {:?}", get_min_distance(intersections, true)),
        _ => unimplemented!(),
    }

    Ok(())
}

#[cfg(test)]
mod test {
    #[test]
    pub fn test_part1() {
        use super::*;

        let example1_wires = parse_wire_infos("example1.txt").unwrap();
        assert_eq!(
            get_min_distance(
                example1_wires[0].get_intersections(&example1_wires[1]),
                false
            ),
            159
        );

        let example2_wires = parse_wire_infos("example2.txt").unwrap();
        assert_eq!(
            get_min_distance(
                example2_wires[0].get_intersections(&example2_wires[1]),
                false
            ),
            135
        );
    }

    #[test]
    pub fn test_part2() {
        use super::*;

        let example1_wires = parse_wire_infos("example1.txt").unwrap();
        assert_eq!(
            get_min_distance(
                example1_wires[0].get_intersections(&example1_wires[1]),
                true
            ),
            610
        );

        let example2_wires = parse_wire_infos("example2.txt").unwrap();
        assert_eq!(
            get_min_distance(
                example2_wires[0].get_intersections(&example2_wires[1]),
                true
            ),
            410
        );
    }
}
