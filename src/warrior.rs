pub struct Warrior {
    pub name: String,
    pub health: i16,
    pub attack: i16,
    pub defense: i16
}

impl Warrior {
    pub fn new(name: &str, health: i16, attack: i16, defense: i16) -> Self {
        Self {
            name: name.to_string(),
            health: health,
            attack: attack,
            defense: defense,
        }
    }

    pub fn choose_class(&mut self, class: i8) {
        match class {
            1 => self.set_class_berserker(),
            2 => self.set_class_tank(),
            3 => self.set_class_wizard(),
            _ => return
        }
    }

    fn set_class_berserker(&mut self) {
        self.health = 1000;
        self.attack = 140;
        self.defense = 30;
    }

    fn set_class_tank(&mut self) {
        self.health = 1200;
        self.attack = 100;
        self.defense = 60;
    }

    fn set_class_wizard(&mut self) {
        self.health = 700;
        self.attack = 200;
        self.defense = 20;
    }
}
