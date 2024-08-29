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
fn main() {
    let mut battle: Battle = get_battle_from_files("inputs/00-example.txt", "outputs/tests/good_output.txt");
    battle = execute_simulation(battle);

    for turn in battle.get_turns() {
        let demon = turn.get_demon_to_fight();
        println!("Turn {}", turn.get_id());
        println!("[DEMON] ID: {}", demon.get_id());
        println!("[DEMON] Cost: {}", demon.get_cost());
        println!("[DEMON] Fragments by Turn: {:?}", demon.get_fragments_by_turn());
        println!("Stamina Start: {}", turn.get_stamina_start());
        println!("Stamina End: {}", turn.get_stamina_end());
        println!("Fragments Start: {}", turn.get_fragments_start());
        println!("Fragments End: {}", turn.get_fragments_end());
        println!("==========================");
    }
}
