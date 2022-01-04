pub struct Player {
    pub health: u32,
    pub mana: Option<u32>,
    pub level: u32,
}

impl Player {
    pub fn revive(&self) -> Option<Player> {
        if self.health > 0 {
            None
        } else {
            Some(Player {
                health: 100,
                level: self.level,
                mana: if self.level >= 10 { Some(100) } else { None },
            })
        }
    }

    pub fn cast_spell(&mut self, mana_cost: u32) -> u32 {
        if self.mana.is_none() {
            self.health = if self.health < mana_cost {
                0
            } else {
                self.health - mana_cost
            };
            0
        } else {
            let mana = self.mana.as_mut().unwrap();
            if *mana < mana_cost {
                0
            } else {
                *mana -= mana_cost;
                mana_cost * 2
            }
        }
    }
}
