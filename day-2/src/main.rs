fn main() {
    let input = std::fs::read_to_string("day-2/input.txt").unwrap();
    part_one(&input);
    part_two(&input);
}

fn part_one(input: &str) {
    let mut sum = 0;
    input.lines().for_each(|line| {
        let game = Game::parse(line);

        if game.is_possible() {
            sum += game.number;
        }
    });

    println!("Part 1: {}", sum);
}

fn part_two(input: &str) {
    let mut sum = 0;
    input.lines().for_each(|line| {
        let game = Game::parse(line);
        sum += game.min_red * game.min_green * game.min_blue;
    });

    println!("Part 2: {}", sum);
}

#[derive(Debug)]
struct Game {
    number: u32,
    rounds: Vec<Round>,
    min_red: u32,
    min_green: u32,
    min_blue: u32,
}

impl Game {
    fn parse(line: &str) -> Game {
        let mut parts = line.split(": ");
        let game_part = parts.next().unwrap();

        let number = game_part
            .split_whitespace()
            .nth(1)
            .unwrap()
            .parse::<u32>()
            .unwrap();
        let rounds_parts = parts.next().unwrap().split("; ");

        let mut rounds = vec![];
        let mut min_red = 0;
        let mut min_green = 0;
        let mut min_blue = 0;

        for part in rounds_parts {
            let round = Round::parse(part);
            
            if round.red > min_red {
                min_red = round.red;
            }

            if round.green > min_green {
                min_green = round.green;
            }

            if round.blue > min_blue {
                min_blue = round.blue;
            }

            rounds.push(round);
        }

        Self { number, rounds, min_red, min_green, min_blue }
    }

    fn is_possible(&self) -> bool {
        for round in &self.rounds {
            if round.red > 12 || round.green > 13 || round.blue > 14 {
                return false;
            }
        }

        true
    }
}

#[derive(Debug)]
struct Round {
    red: u32,
    green: u32,
    blue: u32,
}

impl Round {
    fn parse(line: &str) -> Round {
        let parts = line.split(", ");

        let mut round = Self {
            red: 0,
            green: 0,
            blue: 0,
        };

        for part in parts {
            let mut color_parts = part.split_whitespace();
            let number = color_parts.next().unwrap().parse::<u32>().unwrap();
            let color = color_parts.next().unwrap();

            match color {
                "red" => round.red = number,
                "green" => round.green = number,
                "blue" => round.blue = number,
                _ => panic!("Unknown color: {}", color),
            }
        }

        round
    }
}
