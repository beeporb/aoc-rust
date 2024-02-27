use std::fs;
use std::path::Path;

fn main() {
    
    let PUZZLE_INPUT_PATH = Path::new("/workspaces/aoc-rust/day_1/puzzle_input.txt");

    println!("Opening file {}", PUZZLE_INPUT_PATH.display());

    // Read content of file.

    let file_contents = fs::read_to_string(PUZZLE_INPUT_PATH).expect("Get the content of the PUZZLE_INPUT file.");

    // Split file contents into lines.

    let lines = file_contents.split("\n").collect::<Vec<&str>>();

    let mut total: u32 = 0;

    for line in lines {
        let line_ints: Vec<char> = line.chars().into_iter().filter(|&n| n.is_ascii_digit()).collect();
        let first = line_ints.as_slice().first().expect("Get the first item in the list.");
        let last = line_ints.as_slice().last().expect("Get the last item in the list.");

        let line_val: u32 = String::from_iter(vec![first, last]).parse().unwrap();

        println!("Line: {}; Line Integers: {}, {}; Line Total: {}", line, first, last, line_val);

        total += line_val;
    }

    println!("Total: {}", total);
    
}
