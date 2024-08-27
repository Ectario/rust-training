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
    use rust_training::objects::demon::Demon;
 
    get_battle_from_files("inputs/00-example.txt", "outputs/tests/good_outputs.txt");

    // let mut demon = Demon::new(
    //     1,
    //     500,
    //     3,
    //     50,
    //     5,
    //     vec![10, 15, 20, 25, 30],
    // );

    // println!("ID: {}", demon.get_id());
    // println!("Cost: {}", demon.get_cost());
    // println!("Fragments by Turn: {:?}", demon.get_fragments_by_turn());
    // demon.fight(100, 0);
    // println!("Fragments on Turn 2: {:?}", demon.get_fragments(9));
}
