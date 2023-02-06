use std::fs;

const HANDLE_SMALL_ALPHABHET: u32 = 97;
const HANDLE_LARGE_ALPHABHET: u32 = 38;

fn find_priority(items: (&str, &str)) -> char { 
    let (compartment_one_items, compartment_two_items) = items;
    let mut item: char = ' ';    
    for item_compartment_one in compartment_one_items.chars() {
        for item_compartment_two in compartment_two_items.chars() {
            if item_compartment_one == item_compartment_two {
                item = item_compartment_one;
            }
        }
    }
    item
}

fn main() {
    let rucksack_info: String = fs::read_to_string("./meta/problem_input.txt").unwrap();
    let rucksack_info: Vec<&str> = rucksack_info.split('\n').collect();
    let mut sum_of_priorities: u32 = 0;

    for &rucksack in rucksack_info.iter() {
        let item: char = find_priority(rucksack.split_at(rucksack.len() / 2));
        let item: u32 = item as u32;

        if item >= 65 && item <= 90 {
            sum_of_priorities += item - HANDLE_LARGE_ALPHABHET;
        } else {
            sum_of_priorities += (item - HANDLE_SMALL_ALPHABHET) + 1;
        }
    }
    println!("Sum of Priorities: {}", sum_of_priorities);
}
