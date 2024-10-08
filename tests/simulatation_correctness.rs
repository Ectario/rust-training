use rust_training::utils::simulate::execute_simulation;
use rust_training::utils::parsing::get_battle_from_files;
use rust_training::objects::battle::Battle;
use rust_training::objects::battle::BattleTrait;
use std::cmp;

fn calculate_score_and_test(input_file: &str, output_file: &str, expected_score: usize) {
    let mut battle: Battle = get_battle_from_files(input_file, output_file);
    battle = execute_simulation(battle);
    let mut score = 0;
    for demon in battle.get_killed_demons() {
        let nb_rewards = cmp::min(demon.get_nb_earning_turns(), battle.get_max_turn() - demon.get_death_turn()); 
        for i in 0..nb_rewards {
            score += demon.get_fragments_by_turn()[i];
        }
    }
    assert_eq!(score, expected_score);
    assert_eq!(score, battle.get_fragments());
}

#[cfg(test)]
mod simulation_tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;
    
    #[test]
    fn test_fights_like_in_example() {
        let mut battle: Battle = get_battle_from_files("inputs/00-example.txt", "outputs/tests/good_output.txt");
        battle = execute_simulation(battle);
        for turn in battle.get_turns() {
            match turn.get_id() {
                0 => {
                    assert_eq!(turn.is_fight(), true);
                },
                1 => {
                    assert_eq!(turn.is_fight(), true);
                },
                2 => {
                    assert_eq!(turn.is_fight(), true);
                },
                3 => {
                    assert_eq!(turn.is_fight(), false);
                },
                4 => {
                    assert_eq!(turn.is_fight(), false);
                },
                5 => {
                    assert_eq!(turn.is_fight(), false);
                },
                6 => {
                    assert_eq!(turn.is_fight(), false);
                },
                7 => {
                    assert_eq!(turn.is_fight(), true);
                },
                8 => {
                    assert_eq!(turn.is_fight(), false);
                },
                9 => {
                    assert_eq!(turn.is_fight(), false);
                },
                _ => {
                    panic!("[!] Too much turns.");
                }
            }
        }
    }

    #[test]
    fn test_reward_per_turn_like_in_example() {
        let mut battle: Battle = get_battle_from_files("inputs/00-example.txt", "outputs/tests/good_output.txt");
        battle = execute_simulation(battle);
        for turn in battle.get_turns() {
            match turn.get_id() {
                0 => {
                    assert_eq!(turn.get_fragments_end() - turn.get_fragments_start(), 0);
                },
                1 => {
                    assert_eq!(turn.get_fragments_end() - turn.get_fragments_start(), 14);
                },
                2 => {
                    assert_eq!(turn.get_fragments_end() - turn.get_fragments_start(), 3);
                },
                3 => {
                    assert_eq!(turn.get_fragments_end() - turn.get_fragments_start(), 17);
                },
                4 => {
                    assert_eq!(turn.get_fragments_end() - turn.get_fragments_start(), 7);
                },
                5 => {
                    assert_eq!(turn.get_fragments_end() - turn.get_fragments_start(), 9);
                },
                6 => {
                    assert_eq!(turn.get_fragments_end() - turn.get_fragments_start(), 11);
                },
                7 => {
                    assert_eq!(turn.get_fragments_end() - turn.get_fragments_start(), 18);
                },
                8 => {
                    assert_eq!(turn.get_fragments_end() - turn.get_fragments_start(), 16);
                },
                9 => {
                    assert_eq!(turn.get_fragments_end() - turn.get_fragments_start(), 25);
                },
                _ => {
                    panic!("[!] Too much turns.");
                }
            }
        }
    }

    #[test]
    fn test_accumulated_score_like_in_example() {
        let mut battle: Battle = get_battle_from_files("inputs/00-example.txt", "outputs/tests/good_output.txt");
        battle = execute_simulation(battle);
        for turn in battle.get_turns() {
            match turn.get_id() {
                0 => {
                    assert_eq!(turn.get_fragments_end(), 0);
                },
                1 => {
                    assert_eq!(turn.get_fragments_end(), 14);
                },
                2 => {
                    assert_eq!(turn.get_fragments_end(), 17);
                },
                3 => {
                    assert_eq!(turn.get_fragments_end(), 34);
                },
                4 => {
                    assert_eq!(turn.get_fragments_end(), 41);
                },
                5 => {
                    assert_eq!(turn.get_fragments_end(), 50);
                },
                6 => {
                    assert_eq!(turn.get_fragments_end(), 61);
                },
                7 => {
                    assert_eq!(turn.get_fragments_end(), 79);
                },
                8 => {
                    assert_eq!(turn.get_fragments_end(), 95);
                },
                9 => {
                    assert_eq!(turn.get_fragments_end(), 120);
                },
                _ => {
                    panic!("[!] Too much turns.");
                }
            }
        }
    }

    #[test]
    fn test_waste_detected() {
        let mut battle: Battle = get_battle_from_files("inputs/00-example-waste.txt", "outputs/tests/good_output.txt");
        battle = execute_simulation(battle);
        for turn in battle.get_turns() {
            match turn.get_id() {
                0 => {
                    assert_eq!(turn.get_wasted_stamina(), 0);
                },
                1 => {
                    assert_eq!(turn.get_wasted_stamina(), 9);
                },
                2 => {
                    assert_eq!(turn.get_wasted_stamina(), 0);
                },
                // no more waste to detect
                _ => {
                    break; 
                }
            }
        }
    }

    #[test]
    fn test_final_score_good() {
        calculate_score_and_test("inputs/00-example-waste.txt", "outputs/tests/good_output.txt", 40);
    }

    #[test]
    fn test_final_score_good_2() {
        calculate_score_and_test("inputs/00-example.txt", "outputs/tests/good_output.txt", 120);
    }

}