// Advent of Code 2022, Day 2
#[derive(Debug, Clone)]
enum RPS {
    Rock,
    Paper,
    Scissors,
}

pub fn generator(input: &str) -> String {
    input.to_string()
}

impl RPS {
    // let's write write a function to determine the winner and loser

    pub fn score(&self) -> i32 {
        match self {
            RPS::Rock => 1,
            RPS::Paper => 2,
            RPS::Scissors => 3,
        }
    }

    /// Get the losing matchup
    pub fn get_loss(&self) -> RPS {
        match self {
            RPS::Rock => RPS::Paper,
            RPS::Paper => RPS::Scissors,
            RPS::Scissors => RPS::Rock,
        }
    }

    /// Get the winning matchup
    pub fn get_win(&self) -> RPS {
        match self {
            RPS::Rock => RPS::Scissors,
            RPS::Paper => RPS::Rock,
            RPS::Scissors => RPS::Paper,
        }
    }
}

fn judge(rps1: &RPS, rps2: &RPS) -> usize {
    // determine the winner and loser

    // return the winner, usize from 0 and 1 for the two players, 2 if draw

    match (rps1, rps2) {
        (RPS::Rock, RPS::Rock) => 2,
        (RPS::Rock, RPS::Paper) => 1,
        (RPS::Rock, RPS::Scissors) => 0,
        (RPS::Paper, RPS::Rock) => 0,
        (RPS::Paper, RPS::Paper) => 2,
        (RPS::Paper, RPS::Scissors) => 1,
        (RPS::Scissors, RPS::Rock) => 1,
        (RPS::Scissors, RPS::Paper) => 0,
        (RPS::Scissors, RPS::Scissors) => 2,
    }
}

impl From<char> for RPS {
    fn from(c: char) -> Self {
        match c {
            'A' => RPS::Rock,
            'B' => RPS::Paper,
            'C' => RPS::Scissors,
            'X' => RPS::Rock,
            'Y' => RPS::Paper,
            'Z' => RPS::Scissors,
            _ => panic!("Invalid character"),
        }
    }
}


pub fn part_1(input: &str) -> i32 {
    let mut total_score1 = [0, 0];

    println!("===Part 1===");

    for i in input.lines() {
        // separate the 2 player inputs with a space
        let mut players = i.split_whitespace();

        let (p1, p2) = (
            RPS::from(players.next().unwrap().chars().next().unwrap()),
            RPS::from(players.next().unwrap().chars().next().unwrap()),
        );

        //println!("{:?} vs {:?}", p1, p2);

        let winner = judge(&p1, &p2);
        let mut score = [0, 0];

        match winner {
            0 => {
                score[0] = p1.score() + 6;
                score[1] = p2.score();
            }
            1 => {
                score[0] = p1.score();
                score[1] = p2.score() + 6;
            }
            2 => {
                score[0] = p1.score() + 3;
                score[1] = p2.score() + 3;
            }
            _ => (),
        }

        //println!("Score: {:?}", score);

        total_score1[0] += score[0];
        total_score1[1] += score[1];
    }

    println!("Total score: {:?}", total_score1);
    total_score1[1]

}

pub fn part_2(input: &str) -> i32 {
    let mut total_score2 = [0, 0];

    for i in input.lines() {
        // separate the 2 player inputs with a space
        let mut players = i.split_whitespace();

        let p1 = RPS::from(players.next().unwrap().chars().next().unwrap());
        let p2_char = players.next().unwrap().chars().next().unwrap();

        let p2 = match p2_char {
            // X means p2 loses
            'X' => {
                //println!("p2 loses");
                p1.get_win()
            }
            'Y' => {
                //println!("p2 draws");
                p1.clone()
            }
            'Z' => {
                //println!("p2 wins");
                p1.get_loss()
            }
            _ => panic!("Invalid character"),
        };

        //println!("{:?} vs {:?}", p1, p2);

        let winner = judge(&p1, &p2);
        let mut score = [0, 0];

        match winner {
            0 => {
                score[0] = p1.score() + 6;
                score[1] = p2.score();
            }
            1 => {
                score[0] = p1.score();
                score[1] = p2.score() + 6;
            }
            2 => {
                score[0] = p1.score() + 3;
                score[1] = p2.score() + 3;
            }
            _ => (),
        }

        //println!("Score: {:?}", score);

        total_score2[0] += score[0];
        total_score2[1] += score[1];
    }

    println!("Total score: {:?}", total_score2);
    total_score2[1]


}