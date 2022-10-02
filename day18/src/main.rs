use std::collections::binary_heap::Iter;

fn main() {
    let inputs: Vec<SnailNumber> = PRACTICE
        .lines()
        .map(|line| line.try_into().expect("conversion error"))
        .collect();
}

#[derive(Debug)]
enum SnailNumber {
    Num(u32),
    Pair(Box<(SnailNumber, SnailNumber)>),
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
