use std::fs;

struct Elf <'a> {
    elf_count: u32,
    food: &'a str,
    total_calories: u32
}

impl <'a> Elf <'a> {
    fn find_total_calories(&mut self) -> (u32, u32) {
        let calorie_string: Vec<&str> = self.food.split('\n').collect();

        for &calorie in calorie_string.iter() {
            self.total_calories += calorie.parse::<u32>().unwrap();
        }

        (self.elf_count, self.total_calories)
    }
}

fn main() {
    let problem_input: String = fs::read_to_string("./meta/problem_input.txt").unwrap();
    let problem_input_sep: Vec<&str> = problem_input.split("\n\n").collect();
    let mut calorie_list: Vec<(u32, u32)> = vec![];
    let mut highest_calorie: (u32, u32);

    // Iterates through the list of calories of each elf's and pushes a tuple for each Elf
    for (index, &elf) in problem_input_sep.iter().enumerate() {
        let mut elf = Elf { elf_count: index as u32 + 1, food: elf, total_calories: 0 };
        calorie_list.push(
            elf.find_total_calories()
        );
    }
    
    highest_calorie = calorie_list[0];

    // Find te Elf with the highest calorie
    for &a in calorie_list.iter().skip(1) {
        if highest_calorie.1 < a.1 {
            highest_calorie = a
        }
    }

    println!("The Elf with high calorie is {} carrying {} calories", highest_calorie.0, highest_calorie.1);
}
