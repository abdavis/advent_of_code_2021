use std::cmp::max;
fn main() {
    let prac_num = PRACTICE
        .lines()
        .map(|line| line.try_into().expect("conversion error"))
        .reduce(|acc, next| SnailNumber::add(acc, next))
        .unwrap();
    println!("Magnitude: {}", prac_num.magnitude());

    let num = INPUT
        .lines()
        .map(|line| line.try_into().expect("conversion error"))
        .reduce(|acc, next| SnailNumber::add(acc, next))
        .unwrap();
    println!("Magnitude: {}", num.magnitude());
    let prac_nums: Vec<SnailNumber> = PRACTICE
        .lines()
        .map(|line| line.try_into().expect("foo"))
        .collect();

    let mut maximum = 0;
    for (i, inum) in prac_nums.iter().enumerate() {
        for (j, jnum) in prac_nums.iter().enumerate() {
            if i != j {
                maximum = max(
                    maximum,
                    SnailNumber::add(inum.clone(), jnum.clone()).magnitude(),
                )
            }
        }
    }
    println!("practice max: {maximum}");
    let nums: Vec<SnailNumber> = INPUT
        .lines()
        .map(|line| line.try_into().expect("foo"))
        .collect();

    let mut maximum = 0;
    for (i, inum) in nums.iter().enumerate() {
        for (j, jnum) in nums.iter().enumerate() {
            if i != j {
                maximum = max(
                    maximum,
                    SnailNumber::add(inum.clone(), jnum.clone()).magnitude(),
                )
            }
        }
    }
    println!("practice max: {maximum}");
}

#[derive(Debug, Clone)]
enum SnailNumber {
    Num(u32),
    Pair(Box<(SnailNumber, SnailNumber)>),
}

impl SnailNumber {
    fn magnitude(&self) -> u32 {
        match self {
            Self::Num(n) => *n,
            Self::Pair(pair) => 3 * pair.0.magnitude() + 2 * pair.1.magnitude(),
        }
    }
    fn add(lhs: Self, rhs: Self) -> Self {
        SnailNumber::Pair(Box::new((lhs, rhs))).reduce()
    }
    fn reduce(mut self) -> Self {
        while self.explode(0).0 {}
        while self.split() {
            while self.explode(0).0 {}
        }
        self
    }
    fn explode(&mut self, depth: u8) -> (bool, Option<u32>, Option<u32>) {
        let mut exploded = false;
        let mut borrowed = self;
        let out = match (&mut borrowed, depth) {
            (Self::Pair(pair), 4) => {
                if let (SnailNumber::Num(left), SnailNumber::Num(right)) = pair.as_ref() {
                    exploded = true;
                    (true, Some(*left), Some(*right))
                } else {
                    panic!("Non number pair at depth 4")
                }
            }
            (Self::Pair(pair), _) => {
                let (left, right) = pair.as_mut();
                match left.explode(depth + 1) {
                    (false, _, _) => match right.explode(depth + 1) {
                        (false, _, _) => (false, None, None),
                        (true, Some(left_val), right_val) => {
                            left.resolve_left_exploded(left_val);
                            (true, None, right_val)
                        }
                        anything_else => anything_else,
                    },
                    (true, left_val, Some(right_val)) => {
                        right.resolve_right_exploded(right_val);
                        (true, left_val, None)
                    }
                    anything_else => anything_else,
                }
            }
            (Self::Num(_), _) => (false, None, None),
        };
        if exploded {
            *borrowed = SnailNumber::Num(0)
        }
        out
    }
    fn resolve_left_exploded(&mut self, val: u32) {
        match self {
            Self::Num(num) => *num += val,
            Self::Pair(pair) => pair.1.resolve_left_exploded(val),
        }
    }
    fn resolve_right_exploded(&mut self, val: u32) {
        match self {
            Self::Num(num) => *num += val,
            Self::Pair(pair) => pair.0.resolve_right_exploded(val),
        }
    }
    fn split(&mut self) -> bool {
        match self {
            Self::Num(n) if *n > 9 && *n % 2 == 0 => {
                *self = Self::Pair(Box::new((Self::Num(*n / 2), Self::Num(*n / 2))));
                true
            }
            Self::Num(n) if *n > 9 && *n % 2 == 1 => {
                *self = Self::Pair(Box::new((Self::Num(*n / 2), Self::Num(*n / 2 + 1))));
                true
            }
            Self::Num(_) => false,
            Self::Pair(pair) => pair.0.split() || pair.1.split(),
        }
    }
}

impl TryFrom<&str> for SnailNumber {
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        fn recurse(chars: &mut impl Iterator<Item = char>) -> Result<SnailNumber, ()> {
            if let Some(c) = chars.next() {
                match c {
                    '0'..='9' => Ok(SnailNumber::Num(c.to_digit(10).unwrap())),
                    '[' => {
                        let left = recurse(chars)?;
                        if Some(',') != chars.next() {
                            return Err(());
                        }
                        let right = recurse(chars)?;
                        if Some(']') != chars.next() {
                            return Err(());
                        }
                        Ok(SnailNumber::Pair(Box::new((left, right))))
                    }
                    _ => Err(()),
                }
            } else {
                Err(())
            }
        }
        let mut chars = value.chars();
        let out = recurse(&mut chars)?;
        if None == chars.next() {
            Ok(out)
        } else {
            Err(())
        }
    }

    type Error = ();
}

const PRACTICE: &str = "[[[0,[5,8]],[[1,7],[9,6]]],[[4,[1,2]],[[1,4],2]]]
[[[5,[2,8]],4],[5,[[9,9],0]]]
[6,[[[6,2],[5,6]],[[7,6],[4,7]]]]
[[[6,[0,7]],[0,9]],[4,[9,[9,0]]]]
[[[7,[6,4]],[3,[1,3]]],[[[5,5],1],9]]
[[6,[[7,3],[3,2]]],[[[3,8],[5,7]],4]]
[[[[5,4],[7,7]],8],[[8,3],8]]
[[9,3],[[9,9],[6,[4,9]]]]
[[2,[[7,7],7]],[[5,8],[[9,3],[0,2]]]]
[[[[5,2],5],[8,[3,7]]],[[5,[7,5]],[4,4]]]";

const INPUT: &str = "[[8,[6,[0,1]]],[8,[2,[1,9]]]]
[[[5,[1,7]],9],[4,[[9,4],4]]]
[[[1,[6,3]],8],[[3,[5,3]],[1,6]]]
[0,[[1,[7,4]],[6,[7,3]]]]
[[[7,6],2],[[[9,5],9],[[5,5],[6,3]]]]
[[3,[[2,3],0]],0]
[4,[2,[4,[2,6]]]]
[[2,[8,[4,5]]],[[[7,7],7],1]]
[[[[6,7],6],[9,8]],[[0,[7,3]],[[9,1],[2,0]]]]
[0,[[[8,4],[4,9]],5]]
[[0,7],[8,5]]
[6,[[9,[0,7]],[[0,0],[8,1]]]]
[[[[8,3],[1,9]],[[9,3],[6,5]]],4]
[[[6,6],[[1,2],[1,7]]],[[8,8],[3,2]]]
[[[6,4],[[0,3],1]],[[6,2],[4,[0,3]]]]
[[[2,9],[[2,1],1]],[[6,[1,4]],[6,[0,3]]]]
[9,[[7,4],[1,9]]]
[[[[1,2],[7,8]],[[9,6],[1,3]]],[[0,6],[[3,6],6]]]
[[[1,[8,6]],[2,[3,4]]],[[0,4],[5,[5,7]]]]
[[[5,9],[[1,0],[4,3]]],[[7,[6,7]],[1,[1,5]]]]
[[[[6,8],[5,9]],[5,[4,5]]],4]
[[[[3,4],4],7],[[5,[3,7]],7]]
[[[[3,3],[7,9]],[1,[4,8]]],0]
[[[3,[9,4]],[1,[3,7]]],[[[1,8],1],[6,1]]]
[3,[[[7,5],[4,8]],[7,8]]]
[[7,[2,[2,4]]],[0,[8,[0,3]]]]
[[[[8,5],3],[3,[8,3]]],[1,[0,[7,4]]]]
[[[[7,1],3],[3,4]],[[3,7],[[1,8],[4,8]]]]
[[[3,[9,9]],4],[[4,2],[[4,2],4]]]
[[[5,[9,1]],[[3,5],[1,9]]],7]
[[[[0,8],5],[9,[5,1]]],[[7,0],1]]
[[[0,2],[[1,9],7]],[[0,3],[[0,3],[4,8]]]]
[[[[1,8],0],[[8,6],[7,6]]],[[[1,8],4],[[0,4],[8,3]]]]
[[[1,[2,7]],[[5,4],[5,0]]],[5,[8,[8,4]]]]
[[[[4,4],[7,3]],[4,[2,3]]],[[[6,5],[1,5]],[5,[8,6]]]]
[[[[7,8],4],[9,[4,2]]],[[[1,4],2],[0,7]]]
[[8,4],[1,[2,5]]]
[[[[2,5],4],[7,[0,2]]],[5,3]]
[[3,[[7,4],3]],3]
[[[3,5],[3,[1,4]]],[[[0,8],1],8]]
[[[[1,9],5],[2,[4,8]]],[[[9,2],[0,1]],1]]
[[[6,[1,5]],[[2,2],6]],[[1,[2,6]],5]]
[[[3,2],[9,3]],[[2,1],[4,8]]]
[[[[9,2],7],[[5,9],[1,2]]],[[[3,0],[2,8]],0]]
[[[6,5],[[9,4],3]],[[[6,2],1],[0,7]]]
[[[8,6],1],[9,[1,[0,1]]]]
[[[[5,1],4],[8,[6,8]]],[4,[[1,8],9]]]
[[[[1,1],[8,9]],[2,[0,6]]],3]
[[[1,[8,3]],[[4,3],1]],[[[4,1],[8,6]],8]]
[[8,[[6,2],8]],[[[4,0],8],6]]
[[[[2,2],7],[[9,0],[3,3]]],[[[4,4],0],2]]
[8,[[3,[9,1]],[0,[9,1]]]]
[[[0,[4,2]],[[2,2],[8,7]]],[[6,[4,2]],[1,6]]]
[[3,2],[4,[[6,2],2]]]
[[6,[3,[2,9]]],[[9,[1,5]],[4,4]]]
[[[[7,5],5],8],[1,[0,[2,7]]]]
[[2,[[2,9],[1,6]]],[[[0,1],[0,2]],[4,[3,4]]]]
[[[[8,9],[7,4]],[8,[6,5]]],1]
[[8,9],[[2,[6,9]],[2,8]]]
[[5,1],8]
[[[8,[4,2]],[5,[1,8]]],[[0,[0,6]],[[6,7],9]]]
[[[8,[8,0]],[[8,0],8]],[[[9,9],9],[9,[5,4]]]]
[[[[3,3],5],[5,[9,0]]],[[2,6],[[3,8],[7,1]]]]
[3,[[[1,5],8],5]]
[[[9,8],[4,3]],5]
[[[5,7],[[2,1],6]],[[4,2],[1,[0,2]]]]
[[[[9,3],[9,8]],[[1,0],6]],[[[6,5],2],[[0,3],6]]]
[8,[[[9,8],[2,8]],[1,0]]]
[[8,[5,9]],[[[4,3],6],[[5,1],4]]]
[[0,8],[1,[4,[6,3]]]]
[3,[3,[6,[5,6]]]]
[0,[0,[[8,0],8]]]
[[0,4],[[7,4],[[0,7],1]]]
[7,[[[6,3],[4,0]],1]]
[9,[5,[[5,3],[2,8]]]]
[[7,[[8,3],[1,7]]],[[[2,7],1],[[9,4],[7,1]]]]
[[[0,[7,3]],3],2]
[[1,[[9,0],2]],3]
[[1,[7,[0,1]]],[[1,8],5]]
[3,[5,[4,1]]]
[3,[[[9,8],4],[4,[9,7]]]]
[[2,9],[0,9]]
[[[[7,1],[9,3]],[1,[1,8]]],9]
[[[9,8],[[7,8],3]],[[1,[6,3]],[2,[7,3]]]]
[[[7,3],[1,[5,5]]],[[4,8],[8,[2,5]]]]
[[2,[[6,5],[4,6]]],[[0,3],7]]
[[[4,[9,7]],[[6,1],6]],[[[8,1],6],[[2,5],9]]]
[[[6,0],0],[9,9]]
[[[[1,0],0],[[5,7],9]],[[[7,2],0],[9,6]]]
[[[[5,0],[2,0]],[0,[7,5]]],[[[7,7],[2,4]],8]]
[0,[[9,[3,4]],[[3,4],6]]]
[[[0,8],[[1,5],[3,4]]],[[5,[6,4]],[[2,5],[2,5]]]]
[[8,0],[[2,[7,9]],9]]
[[[3,[7,0]],[3,[8,4]]],2]
[[8,1],[[[8,9],[1,0]],3]]
[[[8,3],[[4,8],4]],[[8,[8,8]],[0,2]]]
[[0,[9,4]],[[6,8],[[7,1],9]]]
[[[[5,3],[2,8]],[8,7]],[9,[[5,9],[5,2]]]]
[2,[4,[[4,3],8]]]
[[[[7,2],[6,4]],7],8]";
