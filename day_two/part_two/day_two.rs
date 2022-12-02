// A - Rock (1 point)
// B - Paper (2 points)
// C - Scissors (3 points)

// X - Loss
// Y - Draw
// Z - Win

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

fn char_to_round_result(result_char: char) -> RoundResult{
    match result_char {
        'X' => RoundResult::Loss,
        'Y' => RoundResult::Draw,
        'Z' => RoundResult::Win,
        _ => panic!("Invalid round result.")
    }
}

fn eval_result(shape_a: char, predetermined_result: RoundResult) -> i32 {
    let result_a = match shape_a {
        'A' => Shape::Rock,
        'B' => Shape::Paper,
        'C' => Shape::Scissors,
        _ => panic!("Invalid hand shape.")
    };

    let mut shape_b = Shape::Rock;

    shape_b = if predetermined_result == RoundResult::Loss {
        // Determine the shape player B would have to play to lose
        match result_a {
            Shape::Rock => Shape::Scissors,
            Shape::Paper => Shape::Rock,
            Shape::Scissors => Shape::Paper,
        }
    } else if predetermined_result == RoundResult::Draw {
        result_a
    } else {
        // Determine the shape player B would have to play to win
        match result_a {
            Shape::Rock => Shape::Paper,
            Shape::Paper => Shape::Scissors,
            Shape::Scissors => Shape::Rock,
        }
    };

    let shape_b_result = shape_to_i32(shape_b);

    shape_b_result + round_result_to_i32(predetermined_result)
}

fn round_result_to_i32(result: RoundResult) -> i32 {
    match result {
        RoundResult::Loss => 0,
        RoundResult::Draw => 3,
        RoundResult::Win => 6,
    }
}

fn shape_to_i32(shape: Shape) -> i32 {
    // Evaluates the point value of a hand shape
    match shape {
        Shape::Rock => 1,
        Shape::Paper => 2,
        Shape::Scissors => 3,
        _ => unimplemented!(), // Rock Paper Scissors only has 3 shapes, maybe RPS 2 will have more - can't wait until it comes out
    }
}

fn eval_row(row: Vec<&str>) -> i32 {
    let shape_a = row[0].chars().nth(0).unwrap();
    let predetermined_result = row[1].chars().nth(0).unwrap();

    let round_result = eval_result(shape_a, char_to_round_result(predetermined_result));

    round_result
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