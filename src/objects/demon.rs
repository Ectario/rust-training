#[derive(Clone)]
pub struct Demon {
    id: usize,
    cost: usize,
    recovery_turns: usize,
    recovery_stamina: usize,
    nb_earning_turns: usize,
    fragments_by_turn: Vec<usize>,
    death_turn: usize
}

impl Demon {
    pub fn new(
        id: usize,
        cost: usize,
        recovery_turns: usize,
        recovery_stamina: usize,
        nb_earning_turns: usize,
        fragments_by_turn: Vec<usize>,
    ) -> Demon {
        // usize::MAX flag meaning that the demon isn't dead
        let death_turn = usize::MAX;
        Demon {
            id,
            cost,
            recovery_turns,
            recovery_stamina,
            nb_earning_turns,
            fragments_by_turn,
            death_turn
        }
    }

    pub fn get_fragments(&self, current_turn: usize) -> usize {
        let death_turn = self.get_death_turn();
        if current_turn >= death_turn && self.is_dead() {
            let earning_turn = current_turn - death_turn;
            if earning_turn < self.get_nb_earning_turns() {
                return self.get_fragments_by_turn()[earning_turn];
            }
        }
        return 0;
    }

    fn is_dead(&self) -> bool {
        self.death_turn != usize::MAX
    }

    // the stamina_player need to be adjusted after this call depending on the output
    pub fn fight(&mut self, stamina_player: usize, current_turn: usize) -> bool {
        if stamina_player >= self.get_cost() {
            self.set_death_turn(current_turn);
            return true;
        }
        return false;
    }

    // return the stamina amount if: the demon is dead AND the number of turns to get the stamina is respected
    pub fn recover_stamina(&self, current_turn: usize) -> usize {
        if self.is_dead() && self.get_recovery_turns() + self.get_death_turn() == current_turn {
            return self.get_recovery_stamina();
        }
        return 0;
    }

    pub fn get_death_turn(&self) -> usize {
        self.death_turn
    }

    fn set_death_turn(&mut self, death_turn: usize) {
        self.death_turn = death_turn;
    }

    pub fn get_recovery_turns(&self) -> usize {
        self.recovery_turns
    }

    pub fn get_recovery_stamina(&self) -> usize {
        self.recovery_stamina
    }

    pub fn get_id(&self) -> usize {
        self.id
    }

    pub fn get_cost(&self) -> usize {
        self.cost
    }

    pub fn get_nb_earning_turns(&self) -> usize {
        self.nb_earning_turns
    }

    pub fn get_fragments_by_turn(&self) -> &Vec<usize> {
        &self.fragments_by_turn
    }

}