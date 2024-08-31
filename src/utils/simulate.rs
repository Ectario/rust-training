use crate::objects::demon::Demon;
use crate::objects::turn::Turn;
use crate::objects::battle::Battle;
use crate::objects::battle::BattleTrait;

pub fn execute_simulation(mut battle: Battle) -> Battle {
    for id_turn in 0..battle.get_max_turn() {

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
    
    return battle;
}