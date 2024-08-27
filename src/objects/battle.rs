use crate::objects::turn::Turn;
use crate::objects::demon::Demon;

pub struct Battle {
    stamina: usize,
    max_stamina: usize,
    current_turn: usize,
    max_turn: usize,
    turns: Vec<Turn>,
    nb_demons: usize,
    demons: Vec<Demon>,
    fragments: usize,
    demons_order: Vec<usize>, 
}

impl Battle {
    pub fn new(
        stamina: usize,
        max_stamina: usize,
        max_turn: usize,
        nb_demons: usize,
        demons: Vec<Demon>,
        demons_order: Vec<usize>, 
    ) -> Battle {
        let turns: Vec<Turn> = vec![];
        let current_turn = 0;
        let fragments = 0;
        Battle {
            stamina,
            max_stamina,
            current_turn,
            max_turn,
            turns,
            nb_demons,
            demons,
            fragments,
            demons_order, 
        }
    }

    pub fn add_fragments(&mut self, fragments: usize) {
        self.fragments += fragments;
    }

    pub fn next_turn(&mut self, turn: Turn) -> bool {
        if self.current_turn < self.max_turn {
            self.current_turn += 1;
            self.turns.push(turn);
            return true;
        }
        return false;
    }

    pub fn get_demon(&self, id: usize) -> Demon {
        self.demons[id].clone()
    }

    pub fn get_demon_id_from_demons_order(&self, next_demon: usize) -> usize {
        self.demons_order[next_demon]
    }

    pub fn get_stamina(&self) -> usize {
        self.stamina
    }

    pub fn add_stamina(&mut self, stamina: usize) {
        self.stamina += stamina;
    }

    pub fn sub_stamina(&mut self, stamina: usize) {
        if self.stamina < stamina {
            panic!("[!] Can't sub stamina, current stamina too low");
        }
        self.stamina -= stamina;
    }

    pub fn get_max_stamina(&self) -> usize {
        self.max_stamina
    }

    pub fn get_current_turn(&self) -> usize {
        self.current_turn
    }

    pub fn get_max_turn(&self) -> usize {
        self.max_turn
    }

    pub fn get_turns(&self) -> &Vec<Turn> {
        &self.turns
    }

    pub fn get_nb_demons(&self) -> usize {
        self.nb_demons
    }

    pub fn get_fragments(&self) -> usize {
        self.fragments
    }

    
}
