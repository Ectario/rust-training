pub struct Turn {
    id: usize,
    stamina_start: usize,
    stamina_end: usize,
    fragments_start: usize,
    fragments_end: usize,
    fight: bool,
    demon_id_fight: usize,
}

impl Turn {
    pub fn new(
        id: usize,
        stamina_start: usize,
        fragments_start: usize,
        demon_id_fight: usize,
    ) -> Turn {
        let fight = false;
        let fragments_end = 0;
        let stamina_end = 0;
        Turn {
            id,
            stamina_start,
            stamina_end,
            fragments_start,
            fragments_end,
            fight,
            demon_id_fight,
        }
    }

    pub fn get_id(&self) -> usize {
        self.id
    }

    pub fn get_stamina_start(&self) -> usize {
        self.stamina_start
    }

    pub fn get_stamina_end(&self) -> usize {
        self.stamina_end
    }

    pub fn set_stamina_end(&mut self, stamina_end: usize) {
        self.stamina_end = stamina_end;
    }

    pub fn get_fragments_start(&self) -> usize {
        self.fragments_start
    }

    pub fn get_fragments_end(&self) -> usize {
        self.fragments_end
    }

    pub fn set_fragments_end(&mut self, fragments_end: usize) {
        self.fragments_end = fragments_end;
    }

    pub fn is_fight(&self) -> bool {
        self.fight
    }

    pub fn set_fight(&mut self, fight: bool) {
        self.fight = fight;
    }

    pub fn get_demon_id_fight(&self) -> usize {
        self.demon_id_fight
    }
}
