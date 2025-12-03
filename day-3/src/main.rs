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
        let digits = find_max(&number_line, count);
        sum += compute_value(&digits);
    }
    sum
}

fn compute_value(digits: &[u8]) -> usize {
    let mut sum = 0;
    for i in 0..digits.len() {
        sum += digits[i] as usize * (10 as usize).pow(digits.len() as u32 - i as u32 - 1);
    }
    sum
}

fn find_max(numbers: &[u8], count: usize) -> Vec<u8> {
    let mut max_digits = Vec::with_capacity(count);
    let mut indices = Vec::with_capacity(count);
    for _ in 0..count {
        max_digits.push(0); // Default digit
        indices.push(0);
    }

    for i in 0..(numbers.len()) {
        let number = numbers[i];

        // Number of digits to come after this one
        let remaining_numbers = numbers.len() - i;

        // Start at the first digit that could get chosen at the current position
        for n in (count - remaining_numbers.min(count))..count {
            if number > max_digits[n] {
                max_digits[n] = number;
                indices[n] = i;
                for m in (n + 1)..count {
                    // Reset all following digits
                    max_digits[m] = 0;
                    indices[m] = 0;
                }
                break; // don't infer the others
            }
        }
    }

    // let old_solution = find_max_old(numbers);

    // if old_solution.0 != max_digits[0] || old_solution.1 != max_digits[1] {
    //     println!("old: {old_solution:?}");

    //     let indices_set: HashSet<usize> = indices.iter().copied().collect();
    //     for (i, digit) in numbers.iter().enumerate() {
    //         if indices_set.contains(&i) {
    //             // Blue ANSI color code
    //             print!("\x1b[34m{}\x1b[0m", digit);
    //         } else {
    //             print!("{}", digit);
    //         }
    //     }
    //     println!(" digits: {max_digits:?}, indices: {indices:?}"); // Newline at the end
    // }

    max_digits
}

// fn find_max_old(numbers: &[u8]) -> (u8, u8) {
//     let mut num_1 = 0;
//     let mut num_2 = 0;

//     for i in 0..(numbers.len()) {
//         let number = numbers[i];
//         let is_last = i == numbers.len() - 1;
//         if number > num_1 && !is_last {
//             num_1 = number;
//             num_2 = 0;
//         } else if number > num_2 {
//             num_2 = number;
//         }
//     }
//     (num_1, num_2)
// }

fn main() {
    let start = Instant::now();
    let secret_code = challenge(12);
    let duration = start.elapsed();
    println!("secret code: {secret_code}");
    println!("time: {duration:?}");
}
