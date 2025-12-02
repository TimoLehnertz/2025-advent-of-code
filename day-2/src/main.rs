use std::fs;

#[derive(Debug)]
struct Range {
    from: usize,
    to: usize,
}

impl From<&str> for Range {
    fn from(value: &str) -> Self {
        let mut split = value.split("-");
        Self {
            from: split.next().unwrap().parse().unwrap(),
            to: split.next().unwrap().parse().unwrap(),
        }
    }
}

fn challenge_1() {
    let contents = fs::read_to_string("src/input.txt").unwrap();
    let ranges = ranges_from_string(&contents);
    let mut sum = 0;
    for range in ranges {
        for id in range.from..=range.to {
            if is_invalid_id(id) {
                sum += id;
                println!("Invalid id: {id}");
            }
        }
    }
    println!("Secret code: {sum}")
}

fn challenge_2() {
    let contents = fs::read_to_string("src/input.txt").unwrap();
    let ranges = ranges_from_string(&contents);
    let mut sum = 0;
    for range in ranges {
        for id in range.from..=range.to {
            if is_invalid_id_2(id) {
                sum += id;
                println!("Invalid id: {id}");
            }
        }
    }
    println!("Secret code: {sum}")
}

fn ranges_from_string(str: &str) -> Vec<Range> {
    str.split(",").map(|s| s.trim()).map(Into::into).collect()
}

fn is_invalid_id(id: usize) -> bool {
    let str = id.to_string();
    if str.len() < 2 || str.len() % 2 == 1 {
        return false;
    }
    str[..str.len() / 2] == str[str.len() / 2..]
}

/// Now, an ID is invalid if it is made only of some sequence of digits repeated at least twice.
/// So, 12341234 (1234 two times), 123123123 (123 three times), 1212121212 (12 five times),
/// and 1111111 (1 seven times) are all invalid IDs.
fn is_invalid_id_2(id: usize) -> bool {
    let str = id.to_string();
    if str.len() < 2 {
        return false;
    }
    for pattern_len in 1..=str.len() / 2 {
        if str.len() % pattern_len != 0 {
            continue;
        }
        let pattern = &str[..pattern_len];
        let mut all_match = true;
        for i in 1..(str.len() / pattern_len) {
            if &str[(i * pattern_len)..((i + 1) * pattern_len)] != pattern {
                all_match = false;
                break;
            }
        }
        if all_match {
            return true;
        }
    }
    false
}

fn main() {
    challenge_2();
}


#[cfg(test)]
pub mod tests {
    use crate::is_invalid_id_2;

    #[test]
    pub fn test_is_invalid_id_2() {
        assert!(is_invalid_id_2(121212));
        assert!(!is_invalid_id_2(1212123));
        assert!(!is_invalid_id_2(1));
        assert!(is_invalid_id_2(11));
        assert!(is_invalid_id_2(11331133));
    }
}