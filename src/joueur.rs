pub struct joueur {
    a_trait: bool,
}

impl joueur {
    pub fn initialisation() -> joueur {
        joueur {
            a_trait: false,
        }
    }

    pub fn donner_le_trait(&mut self) {
        self.a_trait = true;
    }

    pub fn retirer_le_trait(&mut self) {
        self.a_trait = false;
    }

    pub fn le_trait(&self) -> bool {
        return self.a_trait;
    }
}
