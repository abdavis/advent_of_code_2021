fn main() {
    let mut practice = Octos::<10, 10>::from_string(PRACTICE);
    let mut input = Octos::<10, 10>::from_string(INPUT);
    let flash_count: usize = (0..100).map(|_| practice.step()).sum();
    let input_count: usize = (0..100).map(|_| input.step()).sum();
    println!("Practice flash count after 100 steps:{flash_count}");
    println!("Flash count after 100 steps:{input_count}");

    let prac_steps = practice.step_until_syncronized();
    let input_steps = input.step_until_syncronized();

    println!("Practice synchronized at step {}", prac_steps + 100);
    println!("input synchronized at step {}", input_steps + 100);
}

#[derive(Debug)]
struct Octos<const ROWS: usize, const COLS: usize> {
    energy: [[u8; COLS]; ROWS],
}

impl<const ROWS: usize, const COLS: usize> Octos<ROWS, COLS> {
    fn from_string(s: &str) -> Self {
        let mut energy = [[0; COLS]; ROWS];
        for (row, line) in s.lines().enumerate() {
            for (col, char) in line.chars().enumerate() {
                energy[row][col] = char.to_digit(10).unwrap() as u8;
            }
        }
        Self { energy }
    }
    fn step_until_syncronized(&mut self) -> usize {
        let mut step = 0;
        while !self.all_zeros() {
            self.step();
            step += 1;
        }
        step
    }
    fn all_zeros(&self) -> bool {
        for n in self.energy.iter().flatten() {
            if *n != 0 {
                return false;
            }
        }
        true
    }
    fn step(&mut self) -> usize {
        self.energy.iter_mut().flatten().for_each(|n| *n += 1);

        let mut flashed = [[false; COLS]; ROWS];
        let mut flash_count = 0;

        'outside: loop {
            for row in 0..ROWS {
                for col in 0..COLS {
                    if !flashed[row][col] {
                        if self.energy[row][col] > 9 {
                            flashed[row][col] = true;
                            if row > 0 && col > 0 {
                                self.energy[row - 1][col - 1] += 1;
                            }
                            if row > 0 && col < COLS - 1 {
                                self.energy[row - 1][col + 1] += 1;
                            }
                            if row < ROWS - 1 && col > 0 {
                                self.energy[row + 1][col - 1] += 1;
                            }
                            if row < ROWS - 1 && col < COLS - 1 {
                                self.energy[row + 1][col + 1] += 1;
                            }
                            if row > 0 {
                                self.energy[row - 1][col] += 1;
                            }
                            if row < ROWS - 1 {
                                self.energy[row + 1][col] += 1;
                            }
                            if col > 0 {
                                self.energy[row][col - 1] += 1;
                            }
                            if col < COLS - 1 {
                                self.energy[row][col + 1] += 1;
                            }
                            flash_count += 1;

                            continue 'outside;
                        }
                    }
                }
            }
            break;
        }
        self.energy.iter_mut().flatten().for_each(|n| {
            if *n > 9 {
                *n = 0
            }
        });

        flash_count
    }
}

const PRACTICE: &str = "5483143223
2745854711
5264556173
6141336146
6357385478
4167524645
2176841721
6882881134
4846848554
5283751526";

const INPUT: &str = "1254117228
4416873224
8354381553
1372637614
5586538553
7213333427
3571362825
1681126243
8718312138
5254266347";
