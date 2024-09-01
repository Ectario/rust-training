use crate::objects::battle::Battle;
use crate::objects::battle::BattleTrait;
use crate::objects::battle::BattleGenerativeTrait;
use crate::objects::demon::Demon;
use crate::objects::turn::Turn;
use crate::utils::save::save_to_file;
use crate::get_battle_from_only_input;
use kdam::tqdm;

// heuristic right there
pub fn demon_value(demon: Demon) -> i64 {
    let fragments_by_turn = demon.get_fragments_by_turn().clone();
    let end_index = demon.get_nb_earning_turns();
    let interesting_fragments = &fragments_by_turn[0..end_index];
    let total_fragments: f64 = interesting_fragments.iter().map(|&f| f as f64).sum();

    let recovery_stamina = demon.get_recovery_stamina() as f64;
    let cost = demon.get_cost() as f64;
    let recovery_turns = demon.get_recovery_turns() as f64;

    let result = (total_fragments) * ( (recovery_stamina - cost) / recovery_turns);

    let final_result = result.round() as i64;

    return final_result;
}

fn get_affordable_demon(stamina: usize, used_demons: Vec<usize>, last_demon: Option<Demon>, demons: Vec<Demon>) -> Demon {

    // get only the affordable demon NOT already used
    for demon in demons.clone() {
        if !used_demons.iter().any(|&id| id==demon.get_id()) && demon.get_cost() <= stamina {
            return demon.clone();
        }
    }

    // if there is no affordable demon NOT already used then lets wait the last recovery to works -- that can be optimized since there is not the only last demon which give stamina and maybe the last will never give the stamina if we're the end
    if last_demon.is_some() {
        let futur_stamina = last_demon.unwrap().get_recovery_stamina() + stamina;
        for demon in demons.clone() {
            if !used_demons.iter().any(|&id| id==demon.get_id()) && demon.get_cost() <= futur_stamina {
                return demon.clone();
            }
        }
    } else {
        panic!("[!] No affordable demon at first turn?!");
    }
    
    // After the past tries, lets pick the cheapest demon remaining
    let mut affordable_demons: Vec<Demon> = vec![];
    for demon in demons.clone() {
        if !used_demons.iter().any(|&id| id==demon.get_id()) {
            affordable_demons.push(demon.clone());
        }
    }
    if let Some(smallest_demon) = affordable_demons.iter().min_by_key(|demon| demon.get_cost()) {
        return smallest_demon.clone();
    } else {
        panic!("No more demons in the list wtf.");
    }
}

pub fn generate(gen_id: Option<usize>) {
    let input_files = [
        "inputs/00-example.txt",
        "inputs/01-the-cloud-abyss.txt",
        "inputs/02-iot-island-of-terror.txt",
        "inputs/03-etheryum.txt",
        "inputs/04-the-desert-of-autonomous-machines.txt",
        "inputs/05-androids-armageddon.txt"
    ];
    let output_files = [
        "outputs/player/00-example.txt",
        "outputs/player/01-the-cloud-abyss.txt",
        "outputs/player/02-iot-island-of-terror.txt",
        "outputs/player/03-etheryum.txt",
        "outputs/player/04-the-desert-of-autonomous-machines.txt",
        "outputs/player/05-androids-armageddon.txt",
    ];

    if gen_id.is_some() {
        let id: usize = gen_id.unwrap();
        let mut battle: Battle = get_battle_from_only_input(input_files[id]);
        let demons = battle.get_demons();
        let mut last_round_is_fight = false;

        // init for turn 0
        let mut demons_by_value: Vec<Demon> = demons.clone();
        demons_by_value.sort_by_key(|demon| std::cmp::Reverse(demon_value( demon.clone())));
        let mut demon_to_fight: Demon = get_affordable_demon(battle.get_stamina(), battle.get_demons_order(), None, demons.clone());
        battle.add_demons_order(demon_to_fight.get_id());

        for id_turn in tqdm!(0..battle.get_max_turn()) {
            if last_round_is_fight {
                // Logic to know which demon to fight
                let best_demon = get_affordable_demon(battle.get_stamina(), battle.get_demons_order(), Some(demon_to_fight.clone()), demons.clone());
                demon_to_fight = best_demon.clone();
                battle.add_demons_order(demon_to_fight.get_id());
            }

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
                last_round_is_fight = true;
            } else {
                last_round_is_fight = false;
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
            turn.set_demon_end(demon_to_fight.clone());
            turn.set_wasted_stamina_during_turn(wasted_stamina);
            battle.next_turn(turn);
        }
        let _ = save_to_file(output_files[id], battle.get_demons_order());
    } else {
        for i in 0..6 {
            // optimization: threadable & multi process
            generate(Some(i));
        }
    }
}