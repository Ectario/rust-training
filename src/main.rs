mod checks;
mod simulate;
mod parsing;
mod objects {
    pub mod demon;
    pub mod battle;
    pub mod turn;
}

use crate::checks::*;
use crate::simulate::execute_simulation;
use crate::parsing::get_battle_from_files;
use crate::objects::battle::Battle;
use std::env;
use std::assert;

fn help() {
    println!("usage: [command] [input path] [output path]");
}

/*
Input path & output path are required 

-> Check the output format
-> Parse it
-> Simulate the battle

The goal here is to evaluate the score given an input and an output.
Example: cargo run -- simulate ./inputs/00-example.txt outputs/tests/good_output.txt
*/
fn simulate(input_path: &str, output_path: &str) {
    assert!(check_output_charset_and_format(output_path), "[!] Bad charset or format in output path.");
    let mut battle: Battle = get_battle_from_files(input_path, output_path);
    assert!(check_output_range_and_unicity(output_path, battle.get_nb_demons()), "[!] Bad range or a unicity problem occurs in the output file.");
    battle = execute_simulation(battle);

    for turn in battle.get_turns() {
        if turn.get_id() == 0 {
            println!("==========================");
        }
        let demon = turn.get_demon_to_fight();
        println!("Turn {}", turn.get_id());
        println!("[DEMON] ID: {}", demon.get_id());
        println!("[DEMON] Cost: {}", demon.get_cost());
        println!("[DEMON] Fragments by Turn: {:?}", demon.get_fragments_by_turn());
        println!("Fight?: {}", turn.is_fight());
        println!("Stamina Start: {}", turn.get_stamina_start());
        println!("Stamina End: {}", turn.get_stamina_end());
        println!("Stamina Wasted: {}", turn.get_wasted_stamina());
        println!("Fragments Start: {}", turn.get_fragments_start());
        println!("Fragments End: {}", turn.get_fragments_end());
        println!("==========================");
    }
}

/*
Input path & output path are required 

-> Calculate an output file
-> Save it (using the output path)

The goal here is to generate an output file that determines the demon order needed to achieve a good score.
*/
fn generate(input_path: &str, output_path: &str) {

}

/*
Input path & output path are required 

-> Check the output format
-> Parse it
-> Simulate the battle
-> Print only the wasted stamina and the turn associated

The goal here is to evaluate the waste given an input and an output.
Example: cargo run -- waste ./inputs/00-example-waste.txt outputs/tests/good_output.txt
*/
fn waste(input_path: &str, output_path: &str) {
    assert!(check_output_charset_and_format(output_path), "[!] Bad charset or format in output path.");
    let mut battle: Battle = get_battle_from_files(input_path, output_path);
    assert!(check_output_range_and_unicity(output_path, battle.get_nb_demons()), "[!] Bad range or a unicity problem occurs in the output file.");
    battle = execute_simulation(battle);

    let mut total_waste = 0;
    for turn in battle.get_turns() {
        let waste = turn.get_wasted_stamina();
        if waste > 0 {
            if total_waste == 0 {
                println!("==========================");
            }
            println!("Turn {}", turn.get_id());
            total_waste += waste;
            println!("Stamina Wasted: {} (total={})", waste, total_waste);
            println!("==========================");
        }
    }
    if total_waste == 0 {
        println!("[+] No waste detected!");
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    match args.len() {
        // [binary_name] [command] [input path] [output path]
        4 => {
            let cmd = &args[1];
            let input_path = &args[2];
            let output_path = &args[3];
            // parse the command
            match &cmd[..] {
                "simulate" => simulate(input_path, output_path),
                "waste" => waste(input_path, output_path),
                "generate" => generate(input_path, output_path),
                _ => {
                    eprintln!("error: invalid command");
                    help();
                },
            }
        },
        // all the other cases
        _ => {
            // show a help message
            help();
        }
    }
}
