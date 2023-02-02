use std::fs;

macro_rules! enc_to_rpc {
    ($encrypted_msg: expr) => {
        match $encrypted_msg {
            "A" | "X" => RockPaperScissor::Rock,
            "B" | "Y" => RockPaperScissor::Paper,
            "C" | "Z" => RockPaperScissor::Scissor,
            _ => panic!("Unspecified Option")
        }
    };
}

macro_rules! enc_to_result {
    ($encrypted_msg: expr) => {
        match $encrypted_msg {
            "X" => RoundResult::Lose,
            "Y" => RoundResult::Draw,
            "Z" => RoundResult::Win,
            _ => panic!("Unspecified Option")
        }
    };
}

#[derive(PartialEq, Clone, Copy)]
enum RockPaperScissor {
    Rock = 1,
    Paper = 2,
    Scissor = 3
}

enum RoundResult {
    Lose = 0,
    Draw = 3,
    Win = 6
}

struct Game {
    elf_selected: RockPaperScissor,
    human_selected: RockPaperScissor
}

impl Game {
    fn find_round_winner(&self) -> RoundResult {
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
    fn total_points(&self) -> u32 {
        let round_result: RoundResult = self.find_round_winner();
        self.human_selected as u32 + round_result as u32
    }
}

fn convert_to_rpc(elf_selected: RockPaperScissor, human_selected: RoundResult) -> RockPaperScissor {
    match human_selected {
        RoundResult::Lose => {
            if elf_selected == RockPaperScissor::Rock {
                RockPaperScissor::Scissor
            } else if elf_selected == RockPaperScissor::Paper {
                RockPaperScissor::Rock
            } else {
                RockPaperScissor::Paper
            }
        },
        RoundResult::Draw => elf_selected,
        RoundResult::Win => {
            if elf_selected == RockPaperScissor::Rock {
                RockPaperScissor::Paper
            } else if elf_selected == RockPaperScissor::Paper {
                RockPaperScissor::Scissor
            } else {
                RockPaperScissor::Rock
            }
        }
    }
}

fn main() {
    let stategy_guide: String = fs::read_to_string("./meta/problem_input.txt").unwrap();
    let game_rounds:Vec<&str> = stategy_guide.split('\n').collect();
    let mut total_points_part_one: u32 = 0;
    let mut total_points_part_two: u32 = 0;

    for &game_round in game_rounds.iter() {
        let round_collective: Vec<&str> = game_round.split(' ').collect();
        let elf_turn: &str = round_collective.first().unwrap();
        let human_turn: &str = round_collective.last().unwrap();
        let human_turn_rpc:RockPaperScissor = convert_to_rpc(enc_to_rpc!(elf_turn), enc_to_result!(human_turn));

        let round: Game = Game { elf_selected: enc_to_rpc!(elf_turn), human_selected: enc_to_rpc!(human_turn) };
        total_points_part_one += round.total_points();

        let round: Game = Game { elf_selected: enc_to_rpc!(elf_turn), human_selected: human_turn_rpc };
        total_points_part_two += round.total_points();
    }
    println!("Total Points in Part 1: {}", total_points_part_one);
    println!("Total Points in Part 2: {}", total_points_part_two);
}
