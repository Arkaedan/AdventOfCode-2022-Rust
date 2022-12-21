use std::collections::HashMap;
use std::fs;

// --------------------------------------------
// Part 1
// --------------------------------------------

// fn main() {
//     let points = HashMap::from([("A", 0), ("B", 2), ("C", 1), ("X", 1), ("Y", 2), ("Z", 3)]);

//     let contents =
//         fs::read_to_string("input.txt").expect("Should have been able to read input file!");
//     let lines = contents.lines();

//     let mut total = 0;

//     for line in lines {
//         let mut round = line.split_whitespace();
//         let opponent_points = points.get(round.next().unwrap()).unwrap();
//         let my_points = points.get(round.next().unwrap()).unwrap();
//         let score = ((opponent_points + my_points) % 3) * 3 + my_points;
//         total += score;
//     }

//     println!("Total: {}", total);
// }

// --------------------------------------------
// Part 2
// --------------------------------------------

fn main() {
    let points = HashMap::from([("X", 0), ("Y", 1), ("Z", 2), ("A", 2), ("B", 0), ("C", 1)]);

    let contents =
        fs::read_to_string("input.txt").expect("Should have been able to read input file!");
    let lines = contents.lines();

    let mut total = 0;

    for line in lines {
        let mut round = line.split_whitespace();
        let opponent_points = points.get(round.next().unwrap()).unwrap();
        let my_points = points.get(round.next().unwrap()).unwrap();
        let score = (opponent_points + my_points) % 3 + 1 + my_points * 3;
        total += score;
    }

    println!("Total: {}", total);
}
