use std::collections::{hash_map, HashMap, HashSet};

type Map = HashMap<String, Node>;
type Set = HashSet<String>;

fn main() {
    let prac_graph = create_graph(PRACTICE);
    let graph = create_graph(INPUT);
    println!(
        "practice paths:{}",
        find_paths(&"start", &prac_graph, &mut Set::new())
    );
    println!("paths:{}", find_paths(&"start", &graph, &mut Set::new()));
    println!("modified practice paths:{}", find_paths2(&prac_graph));
    println!("modified paths:{}", find_paths2(&graph));
}

fn find_paths2(map: &Map) -> usize {
    fn find_paths_helper(key: &str, map: &Map, set: &Set, mut twice: bool) -> usize {
        if key == "end" {
            return 1;
        }
        if key == "start" {
            return 0;
        }
        let node = map.get(key).unwrap();
        if set.contains(key) {
            if twice {
                return 0;
            }
            twice = true;
        }

        let mut set = set.clone();
        if node.only_once {
            set.insert(key.into());
        }
        let mut count = 0;
        for key in &node.edges {
            count += find_paths_helper(&key, map, &set, twice);
        }
        count
    }

    let node = map.get("start").unwrap();
    let mut count = 0;
    for key in &node.edges {
        count += find_paths_helper(&key, map, &mut Set::new(), false);
    }
    count
}

fn find_paths(key: &str, map: &Map, set: &mut Set) -> usize {
    if key == "end" {
        return 1;
    }
    let node = map.get(key).unwrap();
    if set.contains(key) {
        return 0;
    }
    if node.only_once {
        set.insert(key.into());
    }
    let mut count = 0;
    for key in &node.edges {
        count += find_paths(&key, map, set);
    }
    set.remove(key);
    count
}

fn create_graph(input: &str) -> Map {
    let mut map = Map::new();
    for line in input.lines() {
        let mut iter = line.split('-');
        let a = iter.next().unwrap();
        let b = iter.next().unwrap();
        match map.entry(a.into()) {
            hash_map::Entry::Occupied(mut entry) => entry.get_mut().edges.push(b.into()),
            hash_map::Entry::Vacant(vac) => {
                vac.insert(Node {
                    edges: vec![b.into()],
                    only_once: a.to_lowercase() == a,
                });
            }
        }
        match map.entry(b.into()) {
            hash_map::Entry::Occupied(mut entry) => entry.get_mut().edges.push(a.into()),
            hash_map::Entry::Vacant(vac) => {
                vac.insert(Node {
                    edges: vec![a.into()],
                    only_once: b.to_lowercase() == b,
                });
            }
        }
    }
    map
}

#[derive(Debug)]
struct Node {
    edges: Vec<String>,
    only_once: bool,
}

const PRACTICE: &str = "fs-end
he-DX
fs-he
start-DX
pj-DX
end-zg
zg-sl
zg-pj
pj-he
RW-he
fs-DX
pj-RW
zg-RW
start-pj
he-WI
zg-he
pj-fs
start-RW";

const INPUT: &str = "start-qs
qs-jz
start-lm
qb-QV
QV-dr
QV-end
ni-qb
VH-jz
qs-lm
qb-end
dr-fu
jz-lm
start-VH
QV-jz
VH-qs
lm-dr
dr-ni
ni-jz
lm-QV
jz-dr
ni-end
VH-dr
VH-ni
qb-HE";
