use crate::objects::turn::Turn;
use crate::objects::demon::Demon;

#[allow(dead_code)]
pub struct Battle {
    stamina: usize,
    max_stamina: usize,
    current_turn: usize,
    max_turn: usize,
    turns: Vec<Turn>,
    nb_demons: usize,
    demons: Vec<Demon>,
    killed_demons: Vec<Demon>,
    fragments: usize,
    nb_demons_kill: usize,
    demons_order:Vec<usize>, 
}

pub trait BattleTrait {
    fn add_fragments(&mut self, fragments: usize);
    fn next_turn(&mut self, turn: Turn) -> bool;
    fn get_demon(&self, id: usize) -> Demon;
    fn get_killed_demons(&self) -> Vec<Demon>;
    fn get_demons(&self) -> Vec<Demon>;
    fn get_nb_demons_kill(&self) -> usize;
    fn get_demon_id_from_demons_order(&self, next_demon: usize) -> usize;
    fn get_stamina(&self) -> usize;
    fn add_stamina(&mut self, stamina: usize) -> usize;
    fn sub_stamina(&mut self, stamina: usize);
    fn get_max_stamina(&self) -> usize;
    fn get_max_turn(&self) -> usize;
    fn get_turns(&mut self) -> &mut Vec<Turn>;
    fn get_nb_demons(&self) -> usize;
    fn get_fragments(&self) -> usize;
}

impl BattleTrait for Battle {
    fn add_fragments(&mut self, fragments: usize) {
        self.fragments += fragments;
    }

    fn next_turn(&mut self, mut turn: Turn) -> bool {
        if self.current_turn < self.max_turn {
            if turn.is_fight() {
                self.nb_demons_kill += 1;
                self.killed_demons.push(turn.get_demon_to_fight());
            }
            self.current_turn += 1;
            self.turns.push(turn);
            return true;
        }
        return false;
    }

    fn get_demon(&self, id: usize) -> Demon {
        self.demons[id].clone()
    }

    fn get_killed_demons(&self) -> Vec<Demon> {
        self.killed_demons.clone()
    }

    fn get_demons(&self) -> Vec<Demon> {
        self.demons.clone()
    }

    fn get_nb_demons_kill(&self) -> usize {
        self.nb_demons_kill
    }

    fn get_demon_id_from_demons_order(&self, next_demon: usize) -> usize {
        self.demons_order[next_demon]
    }

    fn get_stamina(&self) -> usize {
        self.stamina
    }

    fn add_stamina(&mut self, stamina: usize) -> usize {
        if self.get_max_stamina() >= self.get_stamina() + stamina {
            self.stamina += stamina;
            return 0;
        }
        let wasted_stamina = (self.get_stamina() + stamina) - self.get_max_stamina();
        self.stamina = self.get_max_stamina();
        return wasted_stamina;
    }

    fn sub_stamina(&mut self, stamina: usize) {
        if self.stamina < stamina {
            panic!("[!] Can't sub stamina, current stamina too low");
        }
        self.stamina -= stamina;
    }

    fn get_max_stamina(&self) -> usize {
        self.max_stamina
    }

    fn get_max_turn(&self) -> usize {
        self.max_turn
    }

    fn get_turns(&mut self) -> &mut Vec<Turn> {
        &mut self.turns
    }

    fn get_nb_demons(&self) -> usize {
        self.nb_demons
    }

    fn get_fragments(&self) -> usize {
        self.fragments
    }
}

pub trait BattleGenerativeTrait {
    fn add_demons_order(&mut self, new_demon: usize);
    fn get_demons_order(&self) -> Vec<usize>;
}

impl BattleGenerativeTrait for Battle {
    fn add_demons_order(&mut self, new_demon: usize) {
        self.demons_order.push(new_demon);
    }

    fn get_demons_order(&self) -> Vec<usize> {
        self.demons_order.clone()
    }
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
        let killed_demons: Vec<Demon> = vec![];
        let current_turn = 0;
        let fragments = 0;
        let nb_demons_kill = 0;
        Battle {
            stamina,
            max_stamina,
            current_turn,
            max_turn,
            turns,
            nb_demons,
            demons,
            killed_demons,
            fragments,
            nb_demons_kill,
            demons_order, 
        }
    }
}