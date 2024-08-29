use crate::objects::demon::Demon;
use crate::objects::turn::Turn;
use crate::objects::battle::Battle;


/*

*/
pub fn execute_simulation(mut battle: Battle){
    for id_turn in 0..battle.get_max_turn() {

        let kill = battle.get_nb_demons_kill();
        let demon_id_fight = battle.get_demon_id_from_demons_order(kill);
        let mut demon_to_fight: Demon = battle.get_demon(demon_id_fight);

        // earn stamina
        let mut earned_stamina = 0;
        for demon in battle.get_killed_demons() {
            earned_stamina += demon.recover_stamina(id_turn);
        }

        battle.add_stamina(earned_stamina);

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
        battle.next_turn(turn);
    }

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
        println!("--------------------------");
    }
    
}