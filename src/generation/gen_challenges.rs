use crate::objects::battle::Battle;
use crate::objects::battle::BattleTrait;
use crate::utils::simulate::execute_simulation;
use crate::utils::save::save_to_file;
use crate::get_battle_from_only_input;
use std::fs;
use rand::thread_rng;
use rand::seq::SliceRandom;
use kdam::tqdm;

#[allow(dead_code)]
pub fn generate_challenges() {
    let input_files = [
        "inputs/01-the-cloud-abyss.txt",
        "inputs/02-iot-island-of-terror.txt",
        "inputs/03-etheryum.txt",
        "inputs/04-the-desert-of-autonomous-machines.txt",
        "inputs/05-androids-armageddon.txt"
    ];
    let output_files = [
        "outputs/challenges/01-the-cloud-abyss.txt",
        "outputs/challenges/02-iot-island-of-terror.txt",
        "outputs/challenges/03-etheryum.txt",
        "outputs/challenges/04-the-desert-of-autonomous-machines.txt",
        "outputs/challenges/05-androids-armageddon.txt"
    ];
    let mut rng = rand::thread_rng();
    for (idx, input_file) in input_files.iter().enumerate() {
        let init_battle: Battle = get_battle_from_only_input(input_file);
        let nb_demons = init_battle.get_nb_demons();
        let initial_stamina = init_battle.get_stamina();
        let max_stamina = init_battle.get_max_stamina();
        let max_turn = init_battle.get_max_turn();
        let demons = init_battle.get_demons();
        let nb_tries = nb_demons / max_turn; // arbitrary but seems interesting to be proportionnal of these values
        let mut range : Vec<usize> = (0..nb_demons).collect();
        let mut record = 0;
        let mut record_order: Vec<usize> = vec![];
        range.shuffle(&mut thread_rng());
        let mut demons_order: Vec<usize> = range[0..max_turn].to_vec();

        for i in tqdm!(0..nb_tries) {
            let mut test_battle = Battle::new(
                initial_stamina,
                max_stamina,
                max_turn,
                nb_demons,
                demons.clone(), 
                demons_order.clone()
            );
            let score = execute_simulation(test_battle).get_fragments();
            if score > record {
                record = score;
                record_order = demons_order.clone();
            }
            range.shuffle(&mut thread_rng());
            demons_order = range[0..max_turn].to_vec();
        }
        let _ = save_to_file(output_files[idx], record_order);
    }
}