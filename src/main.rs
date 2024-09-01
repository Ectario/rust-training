mod objects {
    pub mod demon;
    pub mod battle;
    pub mod turn;
}
mod generation {
    pub mod gen_challenges;
    pub mod gen_player;
}
mod utils {
    pub mod checks;
    pub mod simulate;
    pub mod parsing;
    pub mod save;
}

use crate::utils::checks::*;
use crate::utils::simulate::execute_simulation;
#[allow(unused_imports)]
use crate::generation::gen_challenges::generate_challenges;
use crate::generation::gen_player::generate;
use crate::utils::parsing::get_battle_from_files;
use crate::utils::parsing::get_battle_from_only_input;
use crate::objects::battle::Battle;
use crate::objects::battle::BattleTrait;
use std::env;
use std::assert;

fn help() {
    println!("Usage:");
    println!("  [command] [options]\n");
    
    println!("Commands:");
    println!("  generate                 Generate all outputs");
    println!("  generate [challenge_id]  Generate a specific challenge output based on the given ID.");
    println!("  simulate [input path] [output path]");
    println!("                          Simulate the battle with the given input and output files. Prints the battle state.");
    println!("  waste [input path] [output path]");
    println!("                          Evaluate and print the wasted stamina and the associated turn based on the input and output files.");
    println!("  score [input path] [output path]");
    println!("                          Simulate the battle and print the final score based on the input and output files.\n");
    
    println!("Examples:");
    println!("  cargo run -- generate               # Generate default challenges/outputs");
    println!("  cargo run -- generate 2             # Generate outputs for the 2nd challenge");
    println!("  cargo run -- simulate ./inputs/00-example.txt outputs/tests/good_output.txt");
    println!("                                      # Simulate the battle and print the results");
    println!("  cargo run -- waste ./inputs/00-example-waste.txt outputs/tests/good_output.txt");
    println!("                                      # Evaluate and print wasted stamina");
    println!("  cargo run -- score ./inputs/00-example.txt outputs/tests/good_output.txt");
    println!("                                      # Simulate the battle and print the final score");
}

/*
Input path & output path are required 

-> Check the output format
-> Parse it
-> Simulate the battle
-> Return the final state of the battle

The goal here is to evaluate the score given an input and an output.
Example: cargo run -- simulate ./inputs/00-example.txt outputs/tests/good_output.txt
*/
pub fn simulate(input_path: &str, output_path: &str) -> Battle {
    assert!(check_output_charset_and_format(output_path), "[!] Bad charset or format in output path.");
    let mut battle: Battle = get_battle_from_files(input_path, output_path);
    assert!(check_output_range_and_unicity(output_path, battle.get_nb_demons()), "[!] Bad range or a unicity problem occurs in the output file.");
    battle = execute_simulation(battle);
    return battle;
}

/*
Input path & output path are required 

-> Calculate an output file
-> Save it (using the output path)

The goal here is to generate an output file that determines the demon order needed to achieve a good score.
*/
fn generate_outputs(gen_id: Option<usize>) {
    // generate_challenges();
    generate(gen_id);
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
        // [binary_name] [command]
        2 => {
            let cmd = &args[1];
            // parse the command
            match &cmd[..] {
                "generate" => generate_outputs(None),
                _ => {
                    eprintln!("error: invalid command for this number of args");
                    help();
                },
            }
        },
        // [binary_name] [command] [challenge id]
        3 => {
            let cmd = &args[1];
            let challenge_id = &args[2];
            // parse the command
            match &cmd[..] {
                "generate" => generate_outputs(Some(
                    match challenge_id.parse::<usize>() {
                        Ok(n) => n,
                        Err(_) => panic!("[!] Can't convert {} to usize.", challenge_id),
                      }
                )),
                _ => {
                    eprintln!("error: invalid command for this number of args");
                    help();
                },
            }
        },
        // [binary_name] [command] [input path] [output path]
        4 => {
            let cmd = &args[1];
            let input_path = &args[2];
            let output_path = &args[3];
            // parse the command
            match &cmd[..] {
                "simulate" => {
                    let mut battle: Battle = simulate(input_path, output_path);
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
                    };
                },
                "score" => {
                    let battle: Battle = simulate(input_path, output_path);
                    println!("[+] Score: {}", battle.get_fragments());
                },
                "waste" => waste(input_path, output_path),
                _ => {
                    eprintln!("error: invalid command for this number of args");
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
