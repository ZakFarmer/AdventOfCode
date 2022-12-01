use std::fs::File;
use std::io::{self, BufRead};

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd)]
struct Elf(u32); // Elf struct wrapping their calorie count

fn main() {
    let mut elves = Vec::<Elf>::new();

    if let Ok(input) = read_lines("./puzzle_input.txt") {
        let mut current_total = 0;

        for line in input {
            if let Ok(value) = line {
                if value.len() > 0 {
                    // Use of unwrap() is safe here because we know every populated line is a number
                    current_total += value.parse::<u32>().unwrap();
                    continue;
                }

                // We've reached the end of this elf's snacks, so add the elf's total to the elves vector
                elves.push(Elf(current_total));

                // Set the total back to 0
                current_total = 0;
            }
        }
    }

    // Sort elves in ascending order
    elves.sort();

    let largest = elves.last().unwrap();
    let largest_three = &elves[elves.len() - 3..];

    // Print the Elf with the largest calorie count
    println!("The elf with the most calories has {} calories", largest.0);

    // Print the total of the three largest calorie counts
    println!(
        "The 3 elves with the most calories have: {} calories",
        largest_three.iter().fold(0, |acc, x| acc + x.0)
    );
}

fn read_lines(path: &str) -> Result<io::Lines<io::BufReader<File>>, io::Error> {
    let file = File::open(path)?;
    Ok(io::BufReader::new(file).lines())
}
