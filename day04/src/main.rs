use std::env;

const MIN: u64 = 240_920;
const MAX: u64 = 789_857;

fn has_duplicate(digits: &str) -> bool {
    let mut previous_char_opt = None;
    for c in digits.chars() {
        if let Some(previous_char) = previous_char_opt {
            if previous_char == c {
                return true;
            }
        }

        previous_char_opt = Some(c);
    }

    false
}

fn is_ascending(digits: &str) -> bool {
    let mut previous_char_opt = None;
    for c in digits.chars() {
        if let Some(previous_char) = previous_char_opt {
            if previous_char > c {
                return false;
            }
        }

        previous_char_opt = Some(c);
    }

    true
}

fn is_valid_part1(digits: &str) -> bool {
    has_duplicate(digits) && is_ascending(digits)
}

fn has_duplicate_part2(digits: &str) -> bool {
    let chars = digits.chars().collect::<Vec<char>>();
    let mut i = 0;

    while i < chars.len() - 1 {
        let mut count = 0;

        let mut j = i;
        while j < chars.len() - 1 && chars[j] == chars[j + 1] {
            count += 1;
            j += 1;
        }

        if count == 1 {
            return true;
        } else if count > 0 {
            i = j;
        }

        i += 1;
    }

    false
}

fn is_valid_part2(digits: &str) -> bool {
    has_duplicate_part2(digits) && is_ascending(digits)
}

fn main() {
    let part = env::args().nth(1).expect("Please a part (1 or 2)");

    let mut count = 0;

    for i in MIN..MAX {
        let value = i.to_string();

        let valid = match part.as_str() {
            "1" => is_valid_part1(&value),
            "2" => is_valid_part2(&value),
            _ => unimplemented!(),
        };

        if valid {
            println!("Result: {}", value);
            count += 1;
        }
    }

    println!("Count: {}", count);
}

#[cfg(test)]
mod test {
    #[test]
    pub fn test_part1() {
        use super::is_valid_part1;

        assert!(is_valid_part1("111111"));
        assert!(!is_valid_part1("223450"));
        assert!(!is_valid_part1("123789"));
    }

    #[test]
    pub fn test_part2() {
        use super::is_valid_part2;

        assert!(is_valid_part2("112233"));
        assert!(!is_valid_part2("123444"));
        assert!(is_valid_part2("111122"));
    }
}
