use super::{instructions_from_str, Instructions, INPUT, PRACTICE};
use std::collections::HashMap;

type Polymer = HashMap<(char, char, Option<End>), u64>;

#[derive(PartialEq, Hash, Eq, Debug)]
enum End {
    First,
    Last,
}

pub fn we_can_do_better() {
    let prac_polymer = create_polymer(PRACTICE.lines().next().unwrap());
    let prac_instructions = instructions_from_str(PRACTICE);
    let expanded_prac = expand_n(prac_polymer, &prac_instructions, 10);
    let scored = score(&expanded_prac);
    println!("{scored}");

    let polymer = create_polymer(INPUT.lines().next().unwrap());
    let instructions = instructions_from_str(INPUT);
    let expanded = expand_n(polymer, &instructions, 40);
    let scored = score(&expanded);
    println!("{scored}");
}

fn score(polymer: &Polymer) -> u64 {
    let mut scores = HashMap::<char, u64>::new();

    polymer
        .iter()
        .flat_map(|((a, b, maybe_end), num)| match maybe_end {
            None => [(a, None, num), (b, None, num)],
            Some(End::First) => [(a, Some(End::First), num), (b, None, num)],
            Some(End::Last) => [(a, None, num), (b, Some(End::Last), num)],
        })
        .for_each(|(c, maybe_end, num)| match scores.entry(*c) {
            std::collections::hash_map::Entry::Occupied(mut occ) => match maybe_end {
                None => *occ.get_mut() += num,
                Some(_) => *occ.get_mut() += num * 2,
            },
            std::collections::hash_map::Entry::Vacant(vac) => match maybe_end {
                None => {
                    vac.insert(*num);
                }
                Some(_) => {
                    vac.insert(*num * 2);
                }
            },
        });

    scores.values().max().unwrap() / 2 - scores.values().min().unwrap() / 2
}

fn expand_n(mut polymer: Polymer, instructions: &Instructions, iterations: u64) -> Polymer {
    for _ in 0..iterations {
        polymer = expand(polymer, instructions);
    }
    polymer
}

fn expand(mut polymer: Polymer, instructions: &Instructions) -> Polymer {
    let mut new_poly = Polymer::new();
    polymer
        .drain()
        .flat_map(|((a, b, maybe_end), num)| {
            let c = instructions.get(&(a, b)).unwrap();
            match maybe_end {
                None => [((a, *c, None), num), ((*c, b, None), num)],
                Some(End::First) => [((a, *c, Some(End::First)), num), ((*c, b, None), num)],
                Some(End::Last) => [((a, *c, None), num), ((*c, b, Some(End::Last)), num)],
            }
        })
        .for_each(|(key, num)| match new_poly.entry(key) {
            std::collections::hash_map::Entry::Occupied(mut occ) => *occ.get_mut() += num,
            std::collections::hash_map::Entry::Vacant(vac) => {
                vac.insert(num);
            }
        });
    new_poly
}

fn create_polymer(s: &str) -> Polymer {
    let mut polymer = Polymer::new();
    for (a, b) in s.chars().zip(s.chars().skip(1)).take(s.len() - 2).skip(1) {
        match polymer.entry((a, b, None)) {
            std::collections::hash_map::Entry::Occupied(mut occ) => *occ.get_mut() += 1,
            std::collections::hash_map::Entry::Vacant(vac) => {
                vac.insert(1);
            }
        }
    }
    let mut forward = s.chars();
    let mut backward = s.chars().rev();
    let a = forward.next().unwrap();
    let b = forward.next().unwrap();
    let z = backward.next().unwrap();
    let y = backward.next().unwrap();

    polymer.insert((a, b, Some(End::First)), 1);
    polymer.insert((y, z, Some(End::Last)), 1);

    polymer
}
