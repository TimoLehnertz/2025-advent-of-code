use std::{fs, time::Instant};

fn challenge(count: usize) -> usize {
    let contents = fs::read_to_string("src/input.txt").unwrap();
    let mut number_lines: Vec<Vec<u8>> = Vec::new();
    for line in contents.lines() {
        let mut number_line = Vec::new();
        for c in line.chars() {
            number_line.push(c as u8 - 48);
        }
        number_lines.push(number_line);
    }
    let mut sum: usize = 0;
    for number_line in number_lines {
        let digits = find_max_digits(&number_line, count);
        sum += compute_number_from_digits(&digits);
    }
    sum
}

fn compute_number_from_digits(digits: &[u8]) -> usize {
    let mut sum = 0;
    for i in 0..digits.len() {
        sum += digits[i] as usize * (10 as usize).pow(digits.len() as u32 - i as u32 - 1);
    }
    sum
}

fn find_max_digits(numbers: &[u8], count: usize) -> Vec<u8> {
    let mut max_digits = Vec::with_capacity(count);
    for _ in 0..count {
        max_digits.push(0); // Default digit
    }

    for i in 0..(numbers.len()) {
        let number = numbers[i];

        // Number of digits to come after this one
        let remaining_numbers = numbers.len() - i;

        // Start at the first digit that could get chosen at the current position
        for n in (count - remaining_numbers.min(count))..count {
            if number > max_digits[n] {
                max_digits[n] = number;
                for m in (n + 1)..count {
                    // Reset all following digits
                    max_digits[m] = 0;
                }
                break; // don't infer the others
            }
        }
    }
    max_digits
}

fn main() {
    let start = Instant::now();
    // let secret_code = challenge(2); // challenge 1
    let secret_code = challenge(12); // challenge 2
    let duration = start.elapsed();
    println!("secret code: {secret_code}");
    println!("time: {duration:?}");
}
