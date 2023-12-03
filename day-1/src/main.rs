use std::collections::HashMap;

pub fn main() {
    part_one();
    part_two();
}

fn part_one() {
    let input = std::fs::read_to_string("day-1\\input.txt").unwrap();
    let mut sum = 0;

    input.lines().for_each(|line| {
        let mut left = 0;
        let mut right = 0;

        for i in 0..line.chars().count() {
            let c = line.chars().nth(i).unwrap();
            if c.is_numeric() {
                left = c.to_digit(10).unwrap() * 10;
                break;
            }
        }

        for i in (0..line.chars().count()).rev() {
            let c = line.chars().nth(i).unwrap();
            if c.is_numeric() {
                right = c.to_digit(10).unwrap();
                break;
            }
        }

        sum += left + right;
    });
    println!("Part 1: {sum}");
}

fn part_two() {
    let input = std::fs::read_to_string("day-1\\input.txt").unwrap();
    let value_map = HashMap::<&str, u32>::from([
        ("one", 1),
        ("two", 2),
        ("three", 3),
        ("four", 4),
        ("five", 5),
        ("six", 6),
        ("seven", 7),
        ("eight", 8),
        ("nine", 9),
    ]);
    let mut sum = 0;

    input.lines().for_each(|line| {
        let mut left = 0;
        let mut right = 0;

        'outer: for i in 0..line.chars().count() {
            let c = line.chars().nth(i).unwrap();
            if c.is_numeric() {
                left = c.to_digit(10).unwrap() * 10;
                break;
            }

            for (key, value) in value_map.iter() {
                if i + key.len() > line.chars().count() {
                    continue;
                }

                if line.get(i..i + key.chars().count()).unwrap() == *key {
                    left = *value * 10;
                    break 'outer;
                }
            }
        }

        'outer: for i in (0..line.chars().count()).rev() {
            let c = line.chars().nth(i).unwrap();
            if c.is_numeric() {
                right = c.to_digit(10).unwrap();
                break;
            }

            for (key, value) in value_map.iter() {
                if i + key.len() > line.chars().count() {
                    continue;
                }

                if line.get(i..i + key.chars().count()).unwrap() == *key {
                    right = *value;
                    break 'outer;
                }
            }
        }

        sum += left + right;
    });
    println!("Part 2: {sum}");
}
