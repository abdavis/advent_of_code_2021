use std::collections::{hash_map, HashMap, HashSet};

fn main() {
    let prac_graph = create_graph(PRACTICE);

    println!("{prac_graph:?}")
}

fn create_graph(input: &str) -> HashMap<String, Node> {
    let mut map = HashMap::<String, Node>::new();
    for line in input.lines() {
        let mut iter = line.split('-');
        let a = iter.next().unwrap();
        let b = iter.next().unwrap();
        match map.entry(a.into()) {
            hash_map::Entry::Occupied(mut entry) => entry.get_mut().edges.push(b.into()),
            hash_map::Entry::Vacant(mut vac) => {
                vac.insert(Node {
                    edges: vec![b.into()],
                    only_once: a.to_lowercase() == a,
                });
            }
        }
        match map.entry(b.into()) {
            hash_map::Entry::Occupied(mut entry) => entry.get_mut().edges.push(a.into()),
            hash_map::Entry::Vacant(mut vac) => {
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
