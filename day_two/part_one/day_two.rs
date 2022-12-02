// A, X - Rock (1 point)
// B, Y - Paper (2 points)
// C, Z - Scissors (3 points)

// Loss - 0
// Draw - 3
// Win - 6

use std::fs::File;
use std::io::{self, BufRead};

#[derive(Debug, PartialEq, Eq)]
enum RoundResult {
    Loss,
    Draw,
    Win
}

#[derive(Debug, PartialEq, Eq)]
enum Shape {
    Rock,
    Paper,
    Scissors
}

fn eval_result(shape_a: char, shape_b: char) -> RoundResult {
    let result_a = match shape_a {
        'A' => Shape::Rock,
        'B' => Shape::Paper,
        'C' => Shape::Scissors,
        _ => panic!("Invalid hand shape.")
    };

    let result_b = match shape_b {
        'X' => Shape::Rock,
        'Y' => Shape::Paper,
        'Z' => Shape::Scissors,
        _ => panic!("Invalid hand shape.")
    };

    if result_a == result_b {
        return RoundResult::Draw;
    } else if (result_a == Shape::Rock && result_b == Shape::Scissors) ||
        (result_a == Shape::Paper && result_b == Shape::Rock) ||
        (result_a == Shape::Scissors && result_b == Shape::Paper) {
        return RoundResult::Loss;
    } else {
        return RoundResult::Win;
    }
}

fn round_result_to_i32(result: RoundResult) -> i32 {
    match result {
        RoundResult::Loss => 0,
        RoundResult::Draw => 3,
        RoundResult::Win => 6,
    }
}

fn shape_to_i32(shape: char) -> i32 {
    // Evaluates the point value of a hand shape
    match shape {
        'A' | 'X' => 1,
        'B' | 'Y' => 2,
        'C' | 'Z' => 3,
        _ => unimplemented!(),
    }
}

fn eval_row(row: Vec<&str>) -> i32 {
    let shape_a = row[0].chars().nth(0).unwrap();
    let shape_b = row[1].chars().nth(0).unwrap();

    let shape_result_a = shape_to_i32(shape_a);
    let shape_result_b = shape_to_i32(shape_b);

    let round_result = eval_result(shape_a, shape_b);

    shape_result_b + round_result_to_i32(round_result)
}

fn main() {
    let mut total: i32 = 0;

    if let Ok(input) = read_lines("./puzzle_input.txt") {
        for line in input {
            if let Ok(value) = line {
                let row = value
                    .split(" ")
                    .collect::<Vec<&str>>();

                let result = eval_row(row.clone());

                // println!("Result: {}", result);

                total += result;
            }
        }
    }

    println!("Total score: {}", total);
}

fn read_lines(path: &str) -> Result<io::Lines<io::BufReader<File>>, io::Error> {
    let file = File::open(path)?;
    Ok(io::BufReader::new(file).lines())
}