pub struct Movement {
    pub amount: i32,
    pub from: i32,
    pub to: i32,
}

impl Movement {
    pub fn new(amount: i32, from: i32, to: i32) -> Self {
        Movement { amount, from, to }
    }

    pub fn from_line(line: &str) -> Self {
        let mut parts = line.split_whitespace();

        if parts.next().unwrap() == "move" {
            let amount = parts.next().unwrap().parse::<i32>().unwrap();


            let from = {
                // skip one word
                parts.next();
                let from = parts.next().unwrap();
                // parse i32
                from.parse::<i32>().unwrap() - 1
            };

            let to = {
                // skip one word
                parts.next();
                let to = parts.next().unwrap();
                // parse i32
                to.parse::<i32>().unwrap() - 1
            };

            Movement::new(amount, from, to)
        } else {
            Movement::new(0, 0, 0)
        }
    }
}

// Crane state object, contains the current state of the crane
// Accepts Movement objects to update the state
// The top crate will be at the end of the vector
// [A] [B] [C] [D] [E] [F] [G] [H] [I]
// [J] [K] [L] [M] [N] [O] [P] [Q] [R]
// [1] [2] [3] [4] [5] [6] [7] [8] [9] [10]
// = [[J, A], [K, B], [L, C], [M, D], [N, E], [O, F], [P, G], [Q, H], [R, I]]
pub struct CraneState {
    pub state: Vec<Vec<String>>,
}

impl CraneState {

    pub fn mov(&mut self, m: &Movement) {
        // move n crates from a to b

        // pop the last n crates from a
        let mut row = &mut self.state[m.from as usize];
        let mut crates = row.split_off(row.len() - m.amount as usize);

        for c in crates.iter().rev() {
            self.state[m.to as usize].push(c.to_string());
        }
    }
    pub fn mov2(&mut self, m: &Movement) {
        // move n crates from a to b

        // pop the last n crates from a
        let mut row = &mut self.state[m.from as usize];
        let mut crates = row.split_off(row.len() - m.amount as usize);

        for c in crates.iter() {
            self.state[m.to as usize].push(c.to_string());
        }
    }

    pub fn display(&self) {
        println!("Current state:");

        // get max height of the crane
        let max_height = self.state.iter().map(|x| x.len()).max().unwrap();
        println!("max_height: {:?}", max_height);
        for i in 0..max_height {
            // reverse order, we go from max_height to 0
            let i = max_height - i - 1;
            // println!("i: {:?}", i);
            for row in self.state.iter() {
                if let Some(c) = row.get(i) {
                    print!("{} ", c);
                } else {
                    print!("  ");
                }
            }
            println!();
        }
    }
    pub fn load_str(s: &str) -> CraneState {
        let mut state = Vec::new();
        // the first one is on the top of the stack
        // [A] [B]
        // [C] [D]
        // 1   2

        // get the last line
        let count = s.lines().last().unwrap().split_whitespace();
        for _i in count {
            state.push(Vec::new());
        }

        for line in s.lines().rev().skip(1) {
            // first time, second character.
            // following times, every 4 characters
            // split by 1 space
            // this is very cursed code
            let (_char1, chars) = line.split_at(1);

            // split chars every 4 characters
            let chars = {
                let mut chs = Vec::new();
                // get every 4th character
                for i in chars.chars().step_by(4) {
                    chs.push(i);
                }
                chs
            };

            // println!("line: {:?}", chars);

            (0..state.len()).for_each(|i| {
                if let Some(c) = chars.get(i) {
                    if !c.is_whitespace() {
                        state[i].push(c.to_string());
                    }
                }
            });
        }
        // println!("{:?}", state);

        CraneState { state }
    }
}

pub fn generator(input: &str) -> (&str, &str) {
    input.split_once("\n\n").unwrap()
}

pub fn part_1(input: &(&str, &str)) -> String {
    let (state, instructions) = input;
    let instructions = instructions.lines().map(Movement::from_line).collect::<Vec<_>>();
    let mut s = CraneState::load_str(state);

    for inst in instructions.iter() {
        s.mov(inst);
    }
    // s.display();

    let mut result = String::new();
    for row in s.state.iter() {
        // get the last crate in the row
        if let Some(c) = row.last() {
            // get the ascii value of the crate
            let c = c.chars().next().unwrap();
            // println!("c: {:?}", c);
            result.push(c);
        }
    }
    result
}


pub fn part_2(input: &(&str, &str)) -> String {
    let (state, instructions) = input;
    let instructions = instructions.lines().map(Movement::from_line).collect::<Vec<_>>();
    let mut s = CraneState::load_str(state);

    for inst in instructions.iter() {
        s.mov2(inst);
    }
    // s.display();

    let mut result = String::new();
    for row in s.state.iter() {
        // get the last crate in the row
        if let Some(c) = row.last() {
            // get the ascii value of the crate
            let c = c.chars().next().unwrap();
            // println!("c: {:?}", c);
            result.push(c);
        }
    }
    result
}
