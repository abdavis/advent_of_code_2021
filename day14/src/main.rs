use regex::Regex;
use std::{collections::HashMap, str::FromStr};

mod lets_get_real;

type Instructions = HashMap<(char, char), char>;

fn main() {
    let prac_instructions = instructions_from_str(PRACTICE);
    //println!("{prac_instructions:?}");
    let prac_input = PRACTICE.lines().next().unwrap().to_string();
    println!("{prac_input}");
    let prac_expanded = expand_n(prac_input, &prac_instructions, 10);
    println!("{}", prac_expanded.len());

    let instructions = instructions_from_str(INPUT);
    let input = INPUT.lines().next().unwrap().to_string();

    let out = expand_n(input, &instructions, 10);

    println!("Final string score:{}", score_str(&out));

    lets_get_real::we_can_do_better();
}

fn score_str(s: &str) -> usize {
    let mut map = HashMap::<char, usize>::new();
    for c in s.chars() {
        match map.entry(c) {
            std::collections::hash_map::Entry::Occupied(mut occ) => *occ.get_mut() += 1,
            std::collections::hash_map::Entry::Vacant(vac) => {
                vac.insert(1);
            }
        }
    }

    map.values().max().unwrap() - map.values().min().unwrap()
}

fn expand_n(mut input: String, instructions: &Instructions, iterations: usize) -> String {
    for _ in 0..iterations {
        input = expand(&input, instructions);
    }
    input
}

fn expand(input: &String, instructions: &Instructions) -> String {
    input
        .chars()
        .zip(input.chars().skip(1))
        .flat_map(|(a, b)| [a, *instructions.get(&(a, b)).unwrap()])
        .chain(input.chars().rev().take(1))
        .collect()
}

fn instructions_from_str(s: &str) -> Instructions {
    let mut map = Instructions::new();
    let re = Regex::from_str(r"(\w)(\w) -> (\w)").unwrap();
    for cap in re.captures_iter(s) {
        map.insert(
            (
                cap[1].chars().next().unwrap(),
                cap[2].chars().next().unwrap(),
            ),
            cap[3].chars().next().unwrap(),
        );
    }
    map
}

const PRACTICE: &str = "NNCB

CH -> B
HH -> N
CB -> H
NH -> C
HB -> C
HC -> B
HN -> C
NN -> C
BH -> H
NC -> B
NB -> B
BN -> B
BB -> N
BC -> B
CC -> N
CN -> C";

const INPUT: &str = "PKHOVVOSCNVHHCVVCBOH

NO -> B
PV -> P
OC -> K
SC -> K
FK -> P
PO -> P
FC -> V
KN -> V
CN -> O
CB -> K
NF -> K
CO -> F
SK -> F
VO -> B
SF -> F
PB -> F
FF -> C
HC -> P
PF -> B
OP -> B
OO -> V
OK -> N
KB -> H
PN -> V
PP -> N
FV -> S
BO -> O
HN -> C
FP -> F
BP -> B
HB -> N
VC -> F
PC -> V
FO -> O
OH -> S
FH -> B
HK -> B
BC -> F
ON -> K
FN -> N
NN -> O
PH -> P
KS -> H
HV -> F
BK -> O
NP -> S
CC -> H
KV -> V
NB -> C
NS -> S
KO -> V
NK -> H
HO -> C
KC -> P
VH -> C
VK -> O
CP -> K
BS -> N
BB -> F
VV -> K
SH -> O
SO -> N
VF -> K
NV -> K
SV -> O
NH -> C
VS -> N
OF -> N
SP -> C
HP -> O
NC -> V
KP -> B
KH -> O
SN -> S
CS -> N
FB -> P
OB -> H
VP -> B
CH -> O
BF -> B
PK -> S
CF -> V
CV -> S
VB -> P
CK -> H
PS -> N
SS -> C
OS -> P
OV -> F
VN -> V
BV -> V
HF -> B
FS -> O
BN -> K
SB -> N
HH -> S
BH -> S
KK -> H
HS -> K
KF -> V";
