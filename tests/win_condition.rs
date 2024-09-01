use rust_training::utils::simulate::execute_simulation;
use rust_training::utils::parsing::get_battle_from_files;
use rust_training::objects::battle::Battle;
use rust_training::objects::battle::BattleTrait;

#[cfg(test)]
mod win_tests {
    use super::*;

    fn run_test(input_file: &str, challenge_output_file: &str, player_output_file: &str) {
        let mut battle_challenge: Battle = get_battle_from_files(input_file, challenge_output_file);
        battle_challenge = execute_simulation(battle_challenge);
        println!("[+] Score challenge: {}", battle_challenge.get_fragments());

        let mut battle_player: Battle = get_battle_from_files(input_file, player_output_file);
        battle_player = execute_simulation(battle_player);
        println!("[+] Score player: {}", battle_player.get_fragments());

        assert!(battle_challenge.get_fragments() < battle_player.get_fragments());
    }

    #[test]
    fn test_fight_1() {
        run_test(
            "inputs/01-the-cloud-abyss.txt",
            "outputs/challenges/01-the-cloud-abyss.txt",
            "outputs/player/01-the-cloud-abyss.txt",
        );
    }

    #[test]
    fn test_fight_2() {
        run_test(
            "inputs/02-iot-island-of-terror.txt",
            "outputs/challenges/02-iot-island-of-terror.txt",
            "outputs/player/02-iot-island-of-terror.txt",
        );
    }

    #[test]
    fn test_fight_3() {
        run_test(
            "inputs/03-etheryum.txt",
            "outputs/challenges/03-etheryum.txt",
            "outputs/player/03-etheryum.txt",
        );
    }

    #[test]
    fn test_fight_4() {
        run_test(
            "inputs/04-the-desert-of-autonomous-machines.txt",
            "outputs/challenges/04-the-desert-of-autonomous-machines.txt",
            "outputs/player/04-the-desert-of-autonomous-machines.txt",
        );
    }

    #[test]
    fn test_fight_5() {
        run_test(
            "inputs/05-androids-armageddon.txt",
            "outputs/challenges/05-androids-armageddon.txt",
            "outputs/player/05-androids-armageddon.txt",
        );
    }
}
