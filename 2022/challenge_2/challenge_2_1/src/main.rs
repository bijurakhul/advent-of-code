use std::fs;

macro_rules! find_rpc {
    ($encrypted_msg: expr) => {
        match $encrypted_msg {
            "X" | "A" => RockPaperScissor::Rock,
            "Y" | "B" => RockPaperScissor::Paper,
            "Z" | "C" => RockPaperScissor::Scissor,
            _ => panic!("Unspecified Option")
        }
    };
}

enum RoundResult {
    Lose = 0,
    Draw = 3,
    Win = 6
}

#[derive(PartialEq, Clone, Copy)]
enum RockPaperScissor {
    Rock = 1,
    Paper = 2,
    Scissor = 3
}

struct Game {
    elf_selected: RockPaperScissor,
    human_selected: RockPaperScissor
}

impl Game {
    fn round_winner(&self) -> RoundResult {
        let is_human_winner: bool = (self.elf_selected == RockPaperScissor::Rock && self.human_selected == RockPaperScissor::Paper)
                                            || (self.elf_selected == RockPaperScissor::Paper && self.human_selected == RockPaperScissor::Scissor)
                                            || (self.elf_selected == RockPaperScissor::Scissor && self.human_selected == RockPaperScissor::Rock);
        let is_match_draw: bool = self.elf_selected == self.human_selected;

        if is_human_winner {
            RoundResult::Win
        } else if is_match_draw {
            RoundResult::Draw
        } else {
            RoundResult::Lose
        }
    }
    fn round_total(&self) -> u32 {
        let winner_points = self.round_winner();
        self.human_selected as u32 + winner_points as u32
    }
}

fn main() {
    let stategy_guide: String = fs::read_to_string("./meta/problem_input.txt").unwrap();
    let game_rounds:Vec<&str> = stategy_guide.split('\n').collect();
    let mut total_points: u32 = 0;

    for &game_round in game_rounds.iter() {
        let round_collective: Vec<&str> = game_round.split(' ').collect();
        let elf_turn: &str = round_collective.first().unwrap();
        let human_turn: &str = round_collective.last().unwrap();
        let round: Game = Game { elf_selected: find_rpc!(elf_turn), human_selected: find_rpc!(human_turn) };
        
        total_points += round.round_total();
    }

    println!("{}", total_points);
}
