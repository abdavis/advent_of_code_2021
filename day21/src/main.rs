const INPUT: (usize, usize) = (6, 7);
const PRACTICE: (usize, usize) = (4, 8);
use std::{collections::HashMap, time::Instant};

fn main() {
    println!("{}", get_score(PRACTICE));
    println!("{}", get_score(INPUT));
    multiverse_score(PRACTICE);
    multiverse_score(INPUT);
}

fn multiverse_score(input: (usize, usize)) -> usize {
    fn player_1(
        score_1: &usize,
        pos_1: &usize,
        score_2: &usize,
        pos_2: &usize,
        transposition_table: &mut HashMap<TranspositionKey, (usize, usize)>,
    ) -> (usize, usize) {
        if let Some(val) = transposition_table.get(&TranspositionKey {
            score_1: *score_1,
            pos_1: *pos_1,
            score_2: *score_2,
            pos_2: *pos_2,
            player: Player::One,
        }) {
            return *val;
        }

        let mut wins = (0, 0);
        for i in 1..=3 {
            for j in 1..=3 {
                for k in 1..=3 {
                    let new_pos_1 = (pos_1 + i + j + k) % 10;
                    let new_score_1 = score_1 + new_pos_1 + 1;
                    if new_score_1 >= 21 {
                        wins.0 += 1;
                    } else {
                        let recurse_wins = player_2(
                            &new_score_1,
                            &new_pos_1,
                            score_2,
                            pos_2,
                            transposition_table,
                        );
                        wins.0 += recurse_wins.0;
                        wins.1 += recurse_wins.1;
                    }
                }
            }
        }
        transposition_table.insert(
            TranspositionKey {
                score_1: *score_1,
                pos_1: *pos_1,
                score_2: *score_2,
                pos_2: *pos_2,
                player: Player::One,
            },
            wins,
        );

        wins
    }
    fn player_2(
        score_1: &usize,
        pos_1: &usize,
        score_2: &usize,
        pos_2: &usize,
        transposition_table: &mut HashMap<TranspositionKey, (usize, usize)>,
    ) -> (usize, usize) {
        if let Some(val) = transposition_table.get(&TranspositionKey {
            score_1: *score_1,
            pos_1: *pos_1,
            score_2: *score_2,
            pos_2: *pos_2,
            player: Player::Two,
        }) {
            return *val;
        }
        let mut wins = (0, 0);
        for i in 1..=3 {
            for j in 1..=3 {
                for k in 1..=3 {
                    let new_pos_2 = (pos_2 + i + j + k) % 10;
                    let new_score_2 = score_2 + new_pos_2 + 1;
                    if new_score_2 >= 21 {
                        wins.1 += 1;
                    } else {
                        let recurse_wins = player_1(
                            score_1,
                            pos_1,
                            &new_score_2,
                            &new_pos_2,
                            transposition_table,
                        );
                        wins.0 += recurse_wins.0;
                        wins.1 += recurse_wins.1;
                    }
                }
            }
        }
        transposition_table.insert(
            TranspositionKey {
                score_1: *score_1,
                pos_1: *pos_1,
                score_2: *score_2,
                pos_2: *pos_2,
                player: Player::Two,
            },
            wins,
        );
        wins
    }
    let mut transposition_table = HashMap::new();
    let now = Instant::now();
    let wins = player_1(
        &0,
        &(input.0 - 1),
        &0,
        &(input.1 - 1),
        &mut transposition_table,
    );
    let time = now.elapsed();
    let highest = std::cmp::max(wins.0, wins.1);
    println!(
        "Player 1 Wins: {}, Player 2 Wins: {}, highest: {highest}",
        wins.0, wins.1
    );
    println!("Positions in map: {}", transposition_table.len());
    println!("Time elapsed: {time:?}");
    highest
}

#[derive(Eq, PartialEq, Hash, Clone, Copy)]
struct TranspositionKey {
    score_1: usize,
    pos_1: usize,
    score_2: usize,
    pos_2: usize,
    player: Player,
}
#[derive(Eq, PartialEq, Hash, Clone, Copy)]
enum Player {
    One,
    Two,
}

fn get_score(input: (usize, usize)) -> usize {
    let mut die = (1..=100).cycle().enumerate();
    let mut first_score = 0;
    let mut second_score = 0;
    let mut first_pos = input.0 - 1;
    let mut second_pos = input.1 - 1;
    loop {
        first_pos += die.next().unwrap().1 + die.next().unwrap().1 + die.next().unwrap().1;
        first_pos %= 10;
        first_score += first_pos + 1;
        if first_score >= 1000 {
            println!("first_score: {first_score}, second_score: {second_score}");
            return second_score * die.next().unwrap().0;
        }
        second_pos += die.next().unwrap().1 + die.next().unwrap().1 + die.next().unwrap().1;
        second_pos %= 10;
        second_score += second_pos + 1;
        if second_score >= 1000 {
            println!("first_score: {first_score}, second_score: {second_score}");
            return first_score * die.next().unwrap().0;
        }
    }
}
