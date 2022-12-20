use std::fs;

fn main() {
    let contents =
        fs::read_to_string("input.txt").expect("Should have been able to read input file!");

    let lines = contents.lines();
    let mut elves = Vec::new();
    let mut elf_index = 0;

    elves.push(0);
    for line in lines {
        if line.trim().is_empty() {
            elves.push(0);
            elf_index += 1;
        } else {
            elves[elf_index] += line
                .parse::<i32>()
                .unwrap_or_else(|_| panic!("Should have been able to parse i32 from: {}", line));
        }
    }

    elves.sort_unstable();
    elves.reverse();

    println!("Top One: {}", elves[0]);
    println!("Top Three: {}", elves[0] + elves[1] + elves[2]);
}
