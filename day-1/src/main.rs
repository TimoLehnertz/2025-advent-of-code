use std::time::Instant;

use crate::data::get_data;

pub mod data;

fn main() {
    let start = Instant::now();
    let commands = get_data();
    let mut cursor: i16 = 50;
    let mut zeros: u16 = 0;


    // 0x434C49434B
    let method_2 = true;

    for command in commands {
        let r = command.starts_with("R");
        let number = command[1..].parse::<u32>().unwrap();

        let old_cursor = cursor;

        for _ in 0..number {
            cursor += if r { 1 } else { -1 };
            if cursor == -1 {
                cursor = 99;
            }
            if cursor == 100 {
                cursor = 0;
            }
            if method_2 && cursor == 0 {
                zeros += 1;
            }
        }
        // println!("{command} {old_cursor} > {cursor}");
        if !method_2 && cursor == 0 {
            zeros += 1;
        }
    }
    let duration = start.elapsed();
    println!("Time elapsed: {:?}", duration);
    println!("The secret code is {zeros}");
}
