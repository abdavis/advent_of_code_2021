use std::{
    cmp::{max, min},
    collections::{BinaryHeap, HashSet},
    fmt::Display,
    fmt::Formatter,
    hash::{Hash, Hasher},
    rc::Rc,
    time::Instant,
};

fn main() {
    let start = Instant::now();
    let (_end, _cost) = find_paths::<2>(PRACTICE.into(), end_state());
    let mut time = start.elapsed();
    // println!("{end}");
    // println!("{cost}");

    let start = Instant::now();
    let (_end, _cost) = find_paths::<2>(INPUT.into(), end_state());
    time += start.elapsed();
    // println!("{end}");
    // println!("{cost}");

    let start = Instant::now();
    let first: Node<4> = unfold(PRACTICE).as_str().into();
    let (_end, _cost) = find_paths(first, end_state());
    time += start.elapsed();
    // println!("{end}");
    // println!("{cost}");
    // println!("{} steps", end.depth());

    let start = Instant::now();
    let first: Node<4> = unfold(INPUT).as_str().into();
    let (end, cost) = find_paths(first, end_state());
    time += start.elapsed();
    println!("{end}");
    println!("{cost}");
    println!("{} steps", end.depth());
    println!("Total time for all runs (excluding println calls): {time:?}");
}

fn end_state<const RS: usize>() -> Node<RS> {
    use Space::*;
    Node {
        parent: None,
        hallway: [Empty; 11],
        rooms: [[A; RS], [B; RS], [C; RS], [D; RS]],
    }
}

const PRACTICE: &str = include_str!("practice.txt");

const INPUT: &str = include_str!("input.txt");

const FOLD: &str = "#D#C#B#A#
#D#B#A#C#";

fn unfold(input: &str) -> String {
    input
        .lines()
        .take(3)
        .chain(FOLD.lines())
        .chain(input.lines().skip(3))
        .collect()
}

fn find_paths<const ROOM_SIZE: usize>(
    start: Node<ROOM_SIZE>,
    end: Node<ROOM_SIZE>,
) -> (Rc<Node<ROOM_SIZE>>, usize) {
    let mut queue = BinaryHeap::new();
    let mut completed = HashSet::<Rc<Node<ROOM_SIZE>>>::new();
    queue.push(PriorityKey {
        node: Rc::new(start),
        cost: 0,
    });
    while let Some(key) = queue.pop() {
        if *key.node == end {
            return (key.node, key.cost);
        }
        if !completed.contains(&key.node) {
            completed.insert(key.node.clone());
            queue_children(&key, &mut queue, &completed);
        }
    }

    panic!("Never found end node!")
}

fn queue_children<const ROOM_SIZE: usize>(
    key: &PriorityKey<ROOM_SIZE>,
    queue: &mut BinaryHeap<PriorityKey<ROOM_SIZE>>,
    completed: &HashSet<Rc<Node<ROOM_SIZE>>>,
) {
    use Space::*;
    let child = Node {
        parent: Some(key.node.clone()),
        ..*key.node
    };
    //move to hall
    for (i, room) in key.node.rooms.iter().enumerate() {
        let start = i * 2 + 2;
        let mut move_to_hall = |moving, child: Node<ROOM_SIZE>, steps| {
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
        if let Some((j, spot)) = room.iter().enumerate().find(|(_, s)| **s != Empty) {
            let mut child = child.clone();
            child.rooms[i][j] = Empty;
            if *spot != i.into()
                || (*spot == i.into() && room[j + 1..].iter().any(|s| *s != i.into()))
            {
                move_to_hall(*spot, child, j + 1);
            }
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
        let room_idx = usize::from(space);
        let mut child = child.clone();
        child.hallway[i] = Empty;
        if ((i + 1)..=target)
            .chain(target..i)
            .all(|s| key.node.hallway[s] == Empty)
            && key.node.rooms[room_idx]
                .iter()
                .all(|s| *s == Empty || s == space)
        {
            if let Some((j, _)) = key.node.rooms[room_idx]
                .iter()
                .enumerate()
                .rev()
                .find(|(_, s)| **s == Empty)
            {
                child.rooms[room_idx][j] = *space;
                if !completed.contains(&child) {
                    queue.push(PriorityKey {
                        node: Rc::new(child),
                        cost: (max(i, target) - min(i, target) + j + 1) * space.cost() + key.cost,
                    })
                }
            }
            // match key.node.rooms[room].as_slice() {
            //     [Empty, s, ..] if s == space => {
            //         child.rooms[room][0] = *space;
            //         if !completed.contains(&child) {
            //             queue.push(PriorityKey {
            //                 node: Rc::new(child),
            //                 cost: (max(i, target) - min(i, target) + 1) * space.cost() + key.cost,
            //             })
            //         }
            //     }
            //     [Empty, Empty, ..] => {
            //         child.rooms[room][1] = *space;
            //         if !completed.contains(&child) {
            //             queue.push(PriorityKey {
            //                 node: Rc::new(child),
            //                 cost: (max(i, target) - min(i, target) + 2) * space.cost() + key.cost,
            //             })
            //         }
            //     }
            //     _ => {}
            // }
        }
    }
}

struct PriorityKey<const ROOM_SIZE: usize> {
    node: Rc<Node<ROOM_SIZE>>,
    cost: usize,
}
impl<const ROOM_SIZE: usize> Eq for PriorityKey<ROOM_SIZE> {}
impl<const ROOM_SIZE: usize> PartialEq for PriorityKey<ROOM_SIZE> {
    fn eq(&self, other: &Self) -> bool {
        self.cost == other.cost
    }
}
impl<const ROOM_SIZE: usize> PartialOrd for PriorityKey<ROOM_SIZE> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        //comparing in reverse order because we want a min heap
        Some(other.cost.cmp(&self.cost))
    }
}
impl<const ROOM_SIZE: usize> Ord for PriorityKey<ROOM_SIZE> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        //comparing in reverse order because we want a min heap
        other.cost.cmp(&self.cost)
    }
}

#[derive(Clone)]
struct Node<const ROOM_SIZE: usize> {
    hallway: [Space; 11],
    rooms: [[Space; ROOM_SIZE]; 4],
    parent: Option<Rc<Node<ROOM_SIZE>>>,
}
impl<const ROOM_SIZE: usize> Eq for Node<ROOM_SIZE> {}
impl<const ROOM_SIZE: usize> PartialEq for Node<ROOM_SIZE> {
    fn eq(&self, other: &Self) -> bool {
        self.hallway == other.hallway && self.rooms == other.rooms
    }
}
impl<const ROOM_SIZE: usize> Hash for Node<ROOM_SIZE> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.hallway.hash(state);
        self.rooms.hash(state);
    }
}
impl<const ROOM_SIZE: usize> From<&str> for Node<ROOM_SIZE> {
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

        let mut out = Self {
            hallway: [Space::Empty; 11],
            parent: None,
            rooms: [[Space::Empty; ROOM_SIZE]; 4],
        };
        for i in 0..out.rooms.len() {
            for j in 0..ROOM_SIZE {
                out.rooms[i][j] = letters[j * out.rooms.len() + i];
            }
        }
        out
    }
}

impl<const ROOM_SIZE: usize> Display for Node<ROOM_SIZE> {
    fn fmt(&self, f: &mut Formatter) -> Result<(), std::fmt::Error> {
        if let Some(par) = &self.parent {
            write!(f, "{}", par)?;
        }
        write!(f, "#############\n#")?;
        for s in self.hallway {
            write!(f, "{}", s)?
        }
        write!(f, "#\n")?;
        for n in 0..ROOM_SIZE {
            let padding = if n == 0 { "##" } else { "  " };
            write!(f, "{padding}")?;
            for k in 0..4 {
                write!(f, "#{}", self.rooms[k][n])?;
            }
            write!(f, "#{padding}\n")?;
        }
        write!(f, "  #########\n\n")
    }
}

impl<const ROOM_SIZE: usize> Node<ROOM_SIZE> {
    fn depth(&self) -> usize {
        self.recurse(0)
    }
    fn recurse(&self, depth: usize) -> usize {
        match &self.parent {
            None => depth + 1,
            Some(par) => par.recurse(depth + 1),
        }
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
