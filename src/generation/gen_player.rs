use crate::objects::battle::Battle;
use crate::objects::battle::BattleTrait;
use crate::objects::battle::BattleGenerativeTrait;
use crate::objects::demon::Demon;
use crate::objects::turn::Turn;
use crate::utils::simulate::execute_simulation;
use crate::utils::save::save_to_file;
use crate::get_battle_from_only_input;
use std::fs;
use std::cmp;
use rand::thread_rng;
use rand::seq::SliceRandom;
use kdam::tqdm;

// heuristic right there
fn demon_value(current_turn: usize, max_turn: usize, demon: Demon) -> usize {
    let fragments_by_turn = demon.get_fragments_by_turn().clone();
    let end_index = cmp::min(max_turn-current_turn, demon.get_nb_earning_turns());
    let interesting_fragments = &fragments_by_turn[0..end_index];
    let total_fragments: usize = interesting_fragments.to_vec().iter().sum();
    
    total_fragments * ( ( demon.get_recovery_stamina().pow(2) ) / demon.get_recovery_turns() ) 
}

fn get_affordables_demons(stamina: usize, demons: Vec<Demon>) -> Vec<Demon> {
    let mut affordables_demons: Vec<Demon> = vec![];

    

    return affordables_demons;
}

pub fn generate(gen_id: Option<usize>) {
    let input_files = [
        "inputs/01-the-cloud-abyss.txt",
        "inputs/02-iot-island-of-terror.txt",
        "inputs/03-etheryum.txt",
        "inputs/04-the-desert-of-autonomous-machines.txt",
        "inputs/05-androids-armageddon.txt"
    ];
    let output_files = [
        "outputs/player/01-the-cloud-abyss.txt",
        "outputs/player/02-iot-island-of-terror.txt",
        "outputs/player/03-etheryum.txt",
        "outputs/player/04-the-desert-of-autonomous-machines.txt",
        "outputs/player/05-androids-armageddon.txt"
    ];

    if gen_id.is_some() {
        let id: usize = gen_id.unwrap();
        let mut battle: Battle = get_battle_from_only_input(input_files[id]);
        let demons = battle.get_demons();
        for id_turn in 0..battle.get_max_turn() {
            let mut demons_by_value: Vec<Demon> = get_affordables_demons(battle.get_stamina(), demons.clone());
            demons_by_value.sort_by_key(|demon| std::cmp::Reverse(demon_value(id_turn, battle.get_max_turn(), demon.clone())));
            let kill = battle.get_nb_demons_kill();
            let demon_id_fight = battle.get_demon_id_from_demons_order(kill);
            let mut demon_to_fight: Demon = battle.get_demon(demon_id_fight);
    
            // earn stamina
            let mut earned_stamina = 0;
            for demon in battle.get_killed_demons() {
                earned_stamina += demon.recover_stamina(id_turn);
            }
    
            let wasted_stamina = battle.add_stamina(earned_stamina);
    
            let mut turn = Turn::new(id_turn, battle.get_stamina(), battle.get_fragments(), demon_to_fight.clone());
            
            // if true then FIGHT!
            let player_stamina = battle.get_stamina();
            let cost_fight = demon_to_fight.get_cost();
            if demon_to_fight.fight(player_stamina, id_turn) {
                battle.sub_stamina(cost_fight);
                turn.set_fight(true);
            }
    
            let mut earned_fragments = {
                if turn.is_fight() {
                    demon_to_fight.get_fragments(id_turn)
                } else {
                    0
                }
            };
    
            for demon in battle.get_killed_demons() {
                // earn fragments
                earned_fragments += demon.get_fragments(id_turn);
            }
            battle.add_fragments(earned_fragments);
    
            // set turn ends & save turn
            turn.set_stamina_end(battle.get_stamina());
            turn.set_fragments_end(battle.get_fragments());
            turn.set_demon_end(demon_to_fight);
            turn.set_wasted_stamina_during_turn(wasted_stamina);
            battle.next_turn(turn);
        }
        


















    } else {

    }
}