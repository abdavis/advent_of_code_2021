use std::io::BufRead;

enum Direction {
    Forward(u32),
    Up(u32),
    Down(u32),
}

fn main() {
    let directions: Vec<Direction> = INPUT
        .lines()
        .map(|str| {
            let mut iter = str.split_whitespace();
            match (
                iter.next().unwrap(),
                iter.next().unwrap().parse::<u32>().unwrap(),
            ) {
                ("forward", n) => Direction::Forward(n),
                ("up", n) => Direction::Up(n),
                ("down", n) => Direction::Down(n),
                _ => panic!("malformed input"),
            }
        })
        .collect();
    let (x, y) = calculate_position(&directions);
    println!("x:{}, y:{}, x*y:{}", x, y, x * y);
    println!("revised positions:");
    let (x, y, z) = revised_position(&directions);
    println!("x:{x}, y:{y}, x*y:{}", x * y);
}

fn calculate_position(directions: &Vec<Direction>) -> (u32, u32) {
    directions.iter().fold((0, 0), |acc, dir| match dir {
        Direction::Forward(n) => (acc.0 + n, acc.1),
        Direction::Down(n) => (acc.0, acc.1 + n),
        Direction::Up(n) => (acc.0, acc.1 - n),
    })
}

fn revised_position(directions: &Vec<Direction>) -> (u32, u32, u32) {
    directions.iter().fold((0, 0, 0), |acc, dir| match dir {
        Direction::Down(n) => (acc.0, acc.1, acc.2 + n),
        Direction::Up(n) => (acc.0, acc.1, acc.2 - n),
        Direction::Forward(n) => (acc.0 + n, acc.1 + n * acc.2, acc.2),
    })
}

const INPUT: &str = "forward 6
down 3
forward 8
down 5
forward 9
down 2
up 9
down 9
forward 8
down 3
down 8
forward 2
down 1
up 3
up 6
up 9
down 7
up 7
down 1
forward 7
down 7
up 4
down 2
forward 8
up 3
up 1
down 1
down 6
up 2
down 5
forward 4
down 5
down 3
forward 4
down 3
up 8
forward 1
up 9
forward 2
up 7
down 2
down 9
down 1
down 6
down 8
down 6
down 1
down 1
down 9
down 9
down 2
forward 9
down 1
forward 4
down 2
forward 6
down 8
forward 4
forward 8
forward 4
forward 4
up 4
up 9
down 6
forward 2
forward 5
down 2
forward 1
down 9
forward 2
down 8
down 2
forward 5
down 7
forward 7
down 4
up 3
down 9
forward 3
down 7
up 4
down 5
down 4
forward 8
down 2
down 2
forward 9
down 9
down 5
down 1
down 5
forward 5
down 1
up 7
down 2
forward 7
forward 6
forward 5
forward 4
down 3
forward 9
up 1
down 1
up 8
down 4
down 7
forward 2
down 1
up 9
up 3
down 4
down 1
down 9
down 4
forward 4
forward 7
down 7
down 1
up 6
forward 8
down 8
forward 2
down 4
up 4
forward 3
down 1
up 8
up 2
forward 3
forward 5
forward 7
down 5
up 2
down 6
forward 9
forward 3
down 1
forward 7
up 1
down 4
up 2
forward 5
down 1
forward 2
down 3
forward 9
down 1
down 6
down 7
up 9
down 5
down 1
forward 5
forward 7
down 6
forward 1
down 3
forward 3
forward 1
down 7
forward 9
forward 7
forward 4
up 1
down 8
up 8
down 3
forward 9
up 2
down 4
down 4
down 3
forward 7
forward 3
down 5
up 4
up 7
down 6
forward 2
down 2
down 9
down 9
down 7
down 7
forward 5
forward 8
up 2
forward 9
forward 5
down 2
up 6
down 2
up 2
down 6
down 3
down 2
down 3
down 9
forward 6
up 9
down 3
forward 9
forward 4
forward 1
down 3
down 4
forward 8
forward 4
down 7
forward 9
forward 2
forward 9
down 2
down 3
down 1
down 6
forward 5
down 3
forward 1
down 3
forward 7
down 3
forward 3
up 2
up 8
down 2
down 3
down 7
forward 6
forward 7
up 5
forward 4
forward 6
down 1
forward 1
forward 9
down 2
down 8
forward 6
down 8
down 5
forward 9
forward 3
down 6
forward 3
forward 1
up 7
down 2
down 9
up 6
forward 7
down 9
up 8
forward 5
forward 2
forward 9
down 3
up 7
forward 7
down 4
up 6
up 5
forward 6
forward 2
down 9
forward 9
forward 3
down 4
forward 5
forward 4
forward 4
down 8
forward 4
forward 2
up 8
down 8
forward 6
up 4
down 7
forward 8
up 9
forward 3
forward 5
forward 8
down 5
up 6
up 6
down 5
forward 2
down 3
up 1
down 8
forward 3
down 4
up 9
forward 8
forward 5
forward 2
forward 6
forward 8
up 5
forward 5
down 2
down 4
down 8
forward 3
up 9
down 1
down 9
forward 7
forward 9
down 4
down 2
forward 3
down 1
forward 2
down 2
down 5
forward 2
forward 3
forward 9
down 2
forward 3
forward 9
forward 6
forward 7
down 6
forward 5
up 7
forward 6
up 1
down 7
down 6
down 3
down 7
forward 2
forward 8
forward 3
down 3
forward 7
down 3
up 8
forward 1
down 5
down 9
down 6
forward 1
forward 1
down 1
down 1
forward 8
forward 7
forward 1
up 2
down 4
up 7
down 3
up 8
up 7
forward 3
up 9
down 5
forward 4
down 6
up 8
forward 6
forward 7
down 1
up 7
down 9
down 9
up 9
forward 7
down 6
down 4
down 6
down 7
down 7
up 7
down 4
up 7
forward 1
down 8
down 3
down 2
forward 9
up 7
down 1
down 2
forward 1
forward 5
down 7
up 4
down 7
down 4
down 5
up 8
down 6
down 2
down 4
up 5
down 8
down 3
down 9
forward 6
forward 5
down 1
down 3
down 2
down 3
forward 8
forward 4
forward 6
forward 9
up 1
forward 6
forward 8
down 2
down 1
forward 4
forward 2
forward 3
forward 2
forward 5
forward 2
forward 7
down 5
forward 2
forward 3
forward 9
down 3
down 4
down 7
down 9
down 5
forward 5
down 4
down 8
up 3
forward 1
forward 2
forward 6
up 2
down 9
down 8
up 8
up 3
forward 2
down 6
forward 9
down 3
down 3
forward 7
down 5
forward 2
down 4
down 1
forward 1
down 5
up 4
down 2
forward 8
down 9
down 5
up 4
forward 9
down 3
down 8
forward 8
forward 9
forward 3
up 5
forward 6
down 7
forward 5
down 4
down 9
down 1
up 4
down 8
forward 4
up 4
forward 4
forward 8
forward 3
forward 6
down 9
forward 5
up 4
forward 8
forward 2
down 2
down 1
up 3
forward 5
down 3
down 6
forward 7
down 8
down 1
forward 9
down 8
forward 7
forward 7
forward 7
up 9
up 5
forward 5
forward 2
down 4
up 8
up 7
forward 5
forward 3
forward 7
up 1
down 2
up 1
forward 3
up 8
down 3
forward 1
forward 5
forward 2
forward 5
down 8
up 1
forward 9
down 3
down 7
up 5
down 5
down 1
down 4
down 6
up 9
forward 5
forward 3
down 8
down 7
forward 3
down 9
forward 8
down 3
up 2
up 7
forward 3
down 9
down 5
down 9
up 6
down 9
down 1
down 1
up 4
up 5
up 6
forward 5
down 3
up 1
forward 9
forward 8
forward 8
forward 3
forward 5
forward 8
forward 1
down 8
up 7
down 3
forward 9
forward 1
up 8
down 7
up 4
down 2
down 5
forward 3
down 5
forward 8
forward 4
down 6
up 7
up 7
forward 8
down 6
down 8
down 9
forward 8
forward 1
forward 6
up 2
up 1
up 8
forward 8
forward 1
forward 4
forward 7
forward 2
down 7
down 8
up 5
up 4
up 4
up 7
forward 3
down 2
up 5
down 8
forward 6
up 9
forward 1
down 2
forward 7
down 4
down 6
down 3
down 7
down 9
down 3
forward 1
forward 5
down 2
down 6
up 7
up 2
up 3
up 5
forward 9
down 6
up 1
down 1
forward 3
forward 5
up 8
forward 5
forward 9
up 5
up 4
down 6
up 8
down 8
down 7
down 2
down 6
up 1
up 1
forward 8
down 4
up 3
down 2
down 1
forward 2
down 4
down 6
forward 2
up 8
forward 9
up 1
up 4
forward 2
down 9
down 4
forward 7
forward 6
forward 2
forward 2
forward 5
forward 6
down 3
forward 1
up 9
forward 2
down 3
down 1
down 3
up 9
forward 5
up 5
up 7
down 5
down 4
down 9
down 3
down 3
down 9
down 4
down 3
down 9
forward 9
down 1
down 6
down 7
down 7
down 5
down 8
down 5
forward 1
forward 3
up 1
forward 2
up 5
up 8
down 1
up 8
up 6
up 4
up 1
forward 3
forward 2
forward 4
up 3
down 6
down 1
down 6
up 8
up 7
forward 8
down 9
down 3
forward 2
forward 8
forward 8
down 1
forward 9
down 2
down 3
down 9
down 2
forward 8
down 2
down 6
forward 8
forward 1
up 1
forward 3
down 5
down 6
down 5
down 4
forward 6
forward 3
down 7
down 8
down 7
up 7
down 9
down 8
forward 6
down 1
forward 8
forward 9
up 4
down 1
forward 1
forward 9
down 4
down 2
forward 4
down 5
forward 4
down 7
forward 6
down 3
forward 3
forward 2
forward 7
down 2
forward 2
down 3
up 9
forward 4
forward 1
forward 8
forward 8
forward 6
forward 7
up 8
down 4
up 6
forward 3
up 8
forward 3
forward 1
forward 3
forward 9
up 2
up 5
forward 8
forward 6
forward 6
forward 4
down 6
forward 7
forward 3
forward 2
forward 2
forward 6
forward 5
down 7
up 1
forward 5
up 1
up 9
forward 5
up 3
forward 1
down 2
up 2
down 4
forward 7
forward 4
forward 1
down 1
up 4
down 4
up 2
up 5
down 5
forward 7
up 1
down 6
up 4
forward 3
forward 8
down 6
forward 4
down 2
down 3
down 5
down 4
down 9
up 4
forward 5
up 1
up 2
forward 7
forward 2
up 1
down 8
forward 4
forward 4
up 8
down 3
down 4
up 7
down 8
down 6
down 2
down 3
forward 9
forward 7
forward 6
down 2
down 7
forward 5
forward 2
up 5
down 5
forward 5
down 3
down 1
forward 4
forward 3
down 2
up 1
down 3
down 5
forward 6
forward 5
up 5
down 3
forward 8
down 9
up 4
up 4
down 8
forward 5
down 7
down 3
up 1
down 4
down 5
forward 4
forward 2
forward 4
up 9
down 5
forward 4
forward 6
forward 9
forward 7
forward 5
forward 6
up 4
forward 8
down 4
forward 4
forward 6
up 8
down 4
forward 3
down 8
forward 4
down 9
forward 5
down 4
up 8
forward 2
down 6
up 3
down 5
down 1
down 6
down 9
forward 9
down 1
down 5
up 8
forward 5
down 6
down 9
forward 1
down 6
down 8
down 1
down 2
down 1
forward 5
up 7
forward 5
down 2
down 4
down 1
forward 7
down 7
down 8
forward 4
forward 7
down 2
down 3
forward 2
up 9
down 4
down 5
forward 4
forward 4
forward 6
down 5
forward 8
down 9
forward 8
down 7
up 7
forward 9
up 1
forward 4
up 3
down 2
down 4
down 5
forward 2
forward 8
up 3
up 1
down 1
forward 7
forward 9
forward 6
up 1
down 2
forward 1
up 5
forward 3
down 7
down 6
forward 9
forward 6
forward 3
forward 8
down 2
down 7
forward 1
down 6
up 3
down 6
down 9
up 2
forward 8
forward 1
down 9
forward 8
forward 8
down 3
up 9
down 6
up 3
forward 3
forward 5
forward 7";
