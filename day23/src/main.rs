use std::{
    cmp::{max, min},
    collections::{BinaryHeap, HashSet},
    fmt::Display,
    fmt::Formatter,
    hash::{Hash, Hasher},
    rc::Rc,
};

const END_STATE: Node = Node {
    hallway: [Space::Empty; 11],
    rooms: [[Space::A; 2], [Space::B; 2], [Space::C; 2], [Space::D; 2]],
    parent: None,
};

const PRACTICE: &str = include_str!("practice.txt");

const INPUT: &str = include_str!("input.txt");

fn main() {
    let (end, cost) = find_paths(PRACTICE.into());
    println!("{end}");
    println!("{cost}");

    let (end, cost) = find_paths(INPUT.into());
    //println!("{end}");
    println!("{cost}");
}

fn find_paths(start: Node) -> (Rc<Node>, usize) {
    let mut queue = BinaryHeap::new();
    let mut completed = HashSet::<Rc<Node>>::new();
    queue.push(PriorityKey {
        node: Rc::new(start),
        cost: 0,
    });
    while let Some(key) = queue.pop() {
        if *key.node == END_STATE {
            return (key.node, key.cost);
        }
        if !completed.contains(&key.node) {
            completed.insert(key.node.clone());
            queue_children(&key, &mut queue, &completed);
        }
    }

    panic!("Never found end node!")
}

fn queue_children(
    key: &PriorityKey,
    queue: &mut BinaryHeap<PriorityKey>,
    completed: &HashSet<Rc<Node>>,
) {
    use Space::*;
    let child = Node {
        parent: Some(key.node.clone()),
        ..*key.node
    };
    //move to hall
    for (i, room) in key.node.rooms.iter().enumerate() {
        let start = i * 2 + 2;
        let mut move_to_hall = |moving, child: Node, steps| {
            for n in (0..=start).rev() {
                if key.node.hallway[n] != Empty {
                    break;
                }
                let mut child = child.clone();
                child.hallway[n] = moving;
                if !completed.contains(&child) && n != 2 && n != 4 && n != 6 && n != 8 {
                    queue.push(PriorityKey {
                        node: Rc::new(child),
                        cost: (start - n + steps) * moving.cost() + key.cost,
                    })
                }
            }
            for n in (start)..key.node.hallway.len() {
                if key.node.hallway[n] != Empty {
                    break;
                }
                let mut child = child.clone();
                child.hallway[n] = moving;
                if !completed.contains(&child) && n != 2 && n != 4 && n != 6 && n != 8 {
                    queue.push(PriorityKey {
                        node: Rc::new(child),
                        cost: (n - start + steps) * moving.cost() + key.cost,
                    })
                }
            }
        };

        if room[0] != Empty && (room[0] != i.into() || room[1] != i.into()) {
            let moving = room[0];
            let mut child = child.clone();
            child.rooms[i][0] = Empty;
            move_to_hall(moving, child, 1);
        }
        if room[0] == Empty && room[1] != Empty && room[1] != i.into() {
            let moving = room[1];
            let mut child = child.clone();
            child.rooms[i][1] = Empty;
            move_to_hall(moving, child, 2);
        }
    }
    //move into room
    for (i, space) in key
        .node
        .hallway
        .iter()
        .enumerate()
        .filter(|(_, s)| **s != Empty)
    {
        let target = usize::from(space) * 2 + 2;
        let room = usize::from(space);
        let mut child = child.clone();
        child.hallway[i] = Empty;
        if ((i + 1)..=target)
            .chain(target..i)
            .all(|s| key.node.hallway[s] == Empty)
        {
            match key.node.rooms[room] {
                [Empty, s] if s == *space => {
                    child.rooms[room][0] = *space;
                    if !completed.contains(&child) {
                        queue.push(PriorityKey {
                            node: Rc::new(child),
                            cost: (max(i, target) - min(i, target) + 1) * space.cost() + key.cost,
                        })
                    }
                }
                [Empty, Empty] => {
                    child.rooms[room][1] = *space;
                    if !completed.contains(&child) {
                        queue.push(PriorityKey {
                            node: Rc::new(child),
                            cost: (max(i, target) - min(i, target) + 2) * space.cost() + key.cost,
                        })
                    }
                }
                _ => {}
            }
        }
    }
}

struct PriorityKey {
    node: Rc<Node>,
    cost: usize,
}
impl Eq for PriorityKey {}
impl PartialEq for PriorityKey {
    fn eq(&self, other: &Self) -> bool {
        self.cost == other.cost
    }
}
impl PartialOrd for PriorityKey {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        //comparing in reverse order because we want a min heap
        Some(other.cost.cmp(&self.cost))
    }
}
impl Ord for PriorityKey {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        //comparing in reverse order because we want a min heap
        other.cost.cmp(&self.cost)
    }
}

#[derive(Clone)]
struct Node {
    hallway: [Space; 11],
    rooms: [[Space; 2]; 4],
    parent: Option<Rc<Node>>,
}
impl Eq for Node {}
impl PartialEq for Node {
    fn eq(&self, other: &Self) -> bool {
        self.hallway == other.hallway && self.rooms == other.rooms
    }
}
impl Hash for Node {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.hallway.hash(state);
        self.rooms.hash(state);
    }
}
impl From<&str> for Node {
    fn from(input: &str) -> Self {
        let letters: Vec<Space> = input
            .chars()
            .filter_map(|c| match c {
                'A' => Some(Space::A),
                'B' => Some(Space::B),
                'C' => Some(Space::C),
                'D' => Some(Space::D),
                _ => None,
            })
            .collect();

        Self {
            hallway: [Space::Empty; 11],
            parent: None,
            rooms: [
                [letters[0], letters[4]],
                [letters[1], letters[5]],
                [letters[2], letters[6]],
                [letters[3], letters[7]],
            ],
        }
    }
}

impl Display for Node {
    fn fmt(&self, f: &mut Formatter) -> Result<(), std::fmt::Error> {
        if let Some(par) = &self.parent {
            write!(f, "{}", par)?;
        }
        write!(f, "#############\n#")?;
        for s in self.hallway {
            write!(f, "{}", s)?
        }
        write!(f, "#\n")?;
        let rooms = &self.rooms;
        write!(
            f,
            "###{}#{}#{}#{}###\n  #{}#{}#{}#{}#\n  #########\n\n",
            rooms[0][0],
            rooms[1][0],
            rooms[2][0],
            rooms[3][0],
            rooms[0][1],
            rooms[1][1],
            rooms[2][1],
            rooms[3][1]
        )
    }
}

#[derive(Clone, Copy, Hash, PartialEq, Eq)]
enum Space {
    Empty,
    A,
    B,
    C,
    D,
}

impl Display for Space {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        use Space::*;
        write!(
            f,
            "{}",
            match self {
                A => 'A',
                B => 'B',
                C => 'C',
                D => 'D',
                Empty => '.',
            }
        )
    }
}

impl From<usize> for Space {
    fn from(num: usize) -> Self {
        use Space::*;
        match num {
            0 => A,
            1 => B,
            2 => C,
            3 => D,
            _ => panic!("num is out of range for Space"),
        }
    }
}

impl From<&Space> for usize {
    fn from(space: &Space) -> usize {
        use Space::*;
        match space {
            A => 0,
            B => 1,
            C => 2,
            D => 3,
            Empty => panic!("Empty doesn't have a room!"),
        }
    }
}

impl Space {
    fn cost(&self) -> usize {
        match self {
            Self::A => 1,
            Self::B => 10,
            Self::C => 100,
            Self::D => 1000,
            Self::Empty => panic!("Empty doesn't have a cost!"),
        }
    }
}
