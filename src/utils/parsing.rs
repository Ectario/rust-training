use crate::objects::demon::Demon;
use crate::objects::battle::Battle;
use std::fs;

fn parse_value(value: &str, field_name: &str) -> usize {
    value.parse::<usize>().unwrap_or_else(|_| {
        panic!("[!] In get_battle_from_files: The {} value can't be interpreted.", field_name);
    })
}

fn load_battle(input_file_path: &str, output_file_path: Option<&str>) -> Battle {
    let binding = fs::read_to_string(input_file_path).expect("Should have been able to read the file");
    let mut lines = binding.lines();
    let stamina: usize;
    let max_stamina: usize;
    let max_turn: usize;
    let nb_demons: usize;
    let mut demons: Vec<Demon> = vec![];
    let mut id = 0;

    if let Some(first_line) = lines.next() {
        let values: Vec<&str> = first_line.split_whitespace().collect();
        if values.len() < 4 {
            panic!("[!] The first input line does not contain enough values.");
        }
        stamina = parse_value(values[0], "stamina");
        max_stamina = parse_value(values[1], "max stamina");
        max_turn = parse_value(values[2], "max turns");
        nb_demons = parse_value(values[3], "number of demons");
    } else {
        panic!("[!] Input file empty.");
    }

    for line in lines {
        let values: Vec<&str> = line.split_whitespace().collect();
        if values.len() < 4 {
            panic!("[!] The input line does not contain enough values.");
        }

        let cost = parse_value(values[0], "cost");
        let recovery_turns = parse_value(values[1], "recovery_turns");
        let recovery_stamina = parse_value(values[2], "recovery_stamina");
        let nb_earning_turns = parse_value(values[3], "nb_earning_turns");
        let fragments_by_turn: Vec<usize> = {
            if nb_earning_turns > 0 {
                values[4..]
                    .iter()
                    .map(|&value| parse_value(value, "fragments_by_turn"))
                    .collect()
            } else {
                vec![]
            }
        };

        let created_demon = Demon::new(
            id,
            cost,
            recovery_turns,
            recovery_stamina,
            nb_earning_turns,
            fragments_by_turn,
        );
        demons.push(created_demon);
        id += 1;
    }

    let demons_order: Vec<usize> = if let Some(output_file_path) = output_file_path {
        let binding_output = fs::read_to_string(output_file_path).expect("Should have been able to read the file");
        let values: Vec<&str> = binding_output.lines().collect();
        values.iter().map(|&value| parse_value(value, "demons_order")).collect()
    } else {
        vec![]
    };

    Battle::new(
        stamina,
        max_stamina,
        max_turn,
        nb_demons,
        demons,
        demons_order,
    )
}

pub fn get_battle_from_files(input_file_path: &str, output_file_path: &str) -> Battle {
    load_battle(input_file_path, Some(output_file_path))
}

pub fn get_battle_from_only_input(input_file_path: &str) -> Battle {
    load_battle(input_file_path, None)
}
