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

fn find_paths<const RS: usize>(start: Node<RS>, end: Node<RS>) -> (Rc<Node<RS>>, usize) {
    let mut queue = BinaryHeap::new();
    let mut completed = HashSet::<Rc<Node<RS>>>::new();
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

fn queue_children<const RS: usize>(
    key: &PriorityKey<RS>,
    queue: &mut BinaryHeap<PriorityKey<RS>>,
    completed: &HashSet<Rc<Node<RS>>>,
) {
    use Space::*;
    let child = Node {
        parent: Some(key.node.clone()),
        ..*key.node
    };
    //move to hall
    for (i, room) in key.node.rooms.iter().enumerate() {
        let start = i * 2 + 2;
        let mut move_to_hall = |moving, child: Node<RS>, steps| {
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
        }
    }
}

struct PriorityKey<const RS: usize> {
    node: Rc<Node<RS>>,
    cost: usize,
}
impl<const RS: usize> Eq for PriorityKey<RS> {}
impl<const RS: usize> PartialEq for PriorityKey<RS> {
    fn eq(&self, other: &Self) -> bool {
        self.cost == other.cost
    }
}
impl<const RS: usize> PartialOrd for PriorityKey<RS> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        //comparing in reverse order because we want a min heap
        Some(other.cost.cmp(&self.cost))
    }
}
impl<const RS: usize> Ord for PriorityKey<RS> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        //comparing in reverse order because we want a min heap
        other.cost.cmp(&self.cost)
    }
}

#[derive(Clone)]
struct Node<const RS: usize> {
    hallway: [Space; 11],
    rooms: [[Space; RS]; 4],
    parent: Option<Rc<Node<RS>>>,
}
impl<const RS: usize> Eq for Node<RS> {}
impl<const RS: usize> PartialEq for Node<RS> {
    fn eq(&self, other: &Self) -> bool {
        self.hallway == other.hallway && self.rooms == other.rooms
    }
}
impl<const RS: usize> Hash for Node<RS> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.hallway.hash(state);
        self.rooms.hash(state);
    }
}
impl<const RS: usize> From<&str> for Node<RS> {
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
            rooms: [[Space::Empty; RS]; 4],
        };
        for i in 0..out.rooms.len() {
            for j in 0..RS {
                out.rooms[i][j] = letters[j * out.rooms.len() + i];
            }
        }
        out
    }
}

impl<const RS: usize> Display for Node<RS> {
    fn fmt(&self, f: &mut Formatter) -> Result<(), std::fmt::Error> {
        if let Some(par) = &self.parent {
            write!(f, "{}", par)?;
        }
        write!(f, "#############\n#")?;
        for s in self.hallway {
            write!(f, "{}", s)?
        }
        write!(f, "#\n")?;
        for n in 0..RS {
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

impl<const RS: usize> Node<RS> {
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
