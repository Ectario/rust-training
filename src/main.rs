mod checks;
mod simulate;
mod parsing;
mod objects {
    pub mod demon;
    pub mod battle;
    pub mod turn;
}

use crate::simulate::execute_simulation;
use crate::parsing::get_battle_from_files;
use crate::objects::battle::Battle;

/*
input path & output path needed 

-> check the output format
-> parse it
-> simulate the battle

The goal here is to evaluate the score given an input and an output
*/
fn simulation(){ 
    // execute_simulation();
}

fn main() {
    let mut battle: Battle = get_battle_from_files("inputs/00-example.txt", "outputs/tests/good_output.txt");

    // let mut demon = battle.get_demon(0);

    // println!("ID: {}", demon.get_id());
    // println!("Cost: {}", demon.get_cost());
    // println!("Fragments by Turn: {:?}", demon.get_fragments_by_turn());
    // // demon.fight(100, 0);
    // println!("Fragments on Turn 2: {:?}", demon.get_fragments(2));

    execute_simulation(battle);
}
