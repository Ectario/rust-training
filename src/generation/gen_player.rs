use rust_training::objects::battle::Battle;

pub fn generate(battle: Battle, gen_id: Option<usize>) {
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

    } else {

    }
}