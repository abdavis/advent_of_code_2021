use std::fs;

fn main() {
    let nums: Vec<u32> = fs::read_to_string("input.txt")
        .expect("Couldn't read file")
        .lines()
        .map(|num| num.parse::<u32>().expect("error parsing an integer"))
        .collect();
    let mut prev = nums[0] + nums[1] + nums[2];
    let mut count = 0;
    for n in 1..(nums.len() - 2) {
        let next = nums[n] + nums[n + 1] + nums[n + 2];
        if next > prev {
            count += 1
        }
        prev = next;
    }
    println!("3sum increases {count} times")
}
