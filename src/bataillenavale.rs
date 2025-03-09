pub struct BatailleNavale {
    tab_attaque: [[char; 10]; 10],
    tab_defense: [[char; 10]; 10],
    nombre_de_bateau_mort: u8,
    nombre_de_bateau_vivant: u8,
    nombre_de_case_touche: u32,
    nombre_de_case_coule: u32,
    axe_x_atk: u8,
    axe_y_atk: u8,
    touche: bool,

    // X -> ABCD....
    // Y -> 1234....
}

impl BatailleNavale {
    pub fn initialisation() -> BatailleNavale {
        let mut tab_attaque = [['X'; 10]; 10];
        let mut tab_defense = [['X'; 10]; 10];

        for ligne in tab_defense.iter_mut() {
            ligne[1] = 'B';
        }
        BatailleNavale {
            nombre_de_bateau_vivant: 5,
            nombre_de_bateau_mort: 0,
            nombre_de_case_touche: 0,
            nombre_de_case_coule: 0,
            tab_attaque,
            tab_defense,
            axe_x_atk: 0,
            axe_y_atk: 0,
            touche: false,
        }
    }

    pub fn envoyer_attaque(&mut self, attaque: String) -> bool {
        let tableau_attaque = self.string_splitter(attaque);

        // Si pas valide retourner false
        if !self.verifier_condition(tableau_attaque[0], tableau_attaque[1]) {
            return false;
        }

        let axe_y: usize = ((tableau_attaque[0] as u8) - ('0' as u8)) as usize;
        let axe_x: usize = ((tableau_attaque[1] as u8) - ('A' as u8)) as usize;

        if self.tab_attaque[axe_y][axe_x] == 'X' {
            return true;
        } else {
            return false;
        }

        return false;
    }

    pub fn recevoir_attaque(&self, attaque: String) -> char {
        //On verif si l'attaque est valide
        let tableau_attaque = self.string_splitter(attaque);

        //Si pas valide renvoyer E
        if !self.verifier_condition(tableau_attaque[0], tableau_attaque[1]) {
            return 'E';
        }

        let axe_y: usize = ((tableau_attaque[0] as u8) - ('0' as u8)) as usize;
        let axe_x: usize = ((tableau_attaque[1] as u8) - ('A' as u8)) as usize;

        if self.tab_defense[axe_y][axe_x] == 'X' {
            // Envoyer C
            return 'C';
        } else if self.tab_defense[axe_y][axe_x] == 'B' {
            // Envoyer B
            return 'B';
        } else {
            // Envoyer E;
            return 'E';
        }

        return 'E';
    }

    pub fn lire_case_defense(&self, y: char, x: char) -> char {
        if !self.verifier_condition(y, x) {
            return 'E';
        }

        let axe_y: usize = ((y as u8) - ('0' as u8)) as usize;
        let axe_x: usize = ((x as u8) - ('A' as u8)) as usize;

        print!("{}", axe_x);
        print!("{}", axe_y);
        return self.tab_defense[axe_y][axe_x];
    }

    pub fn lire_case_attaque(&self, y: char, x: char) -> char {
        if !self.verifier_condition(y, x) {
            return 'E';
        }

        let axe_y: usize = ((y as u8) - ('0' as u8)) as usize;
        let axe_x: usize = ((x as u8) - ('A' as u8)) as usize;

        print!("{}", axe_x);
        print!("{}", axe_y);
        return self.tab_attaque[axe_y][axe_x];
    }

    pub fn afficher_champ_bataille(&self) {
        println!("Champ attaque :");
        for ligne in &self.tab_attaque {
            for element in ligne {
                print!("{} ", element);
            }
            println!();
        }
    }

    pub fn afficher_champ_defense(&self) {
        println!("Champ defense :");
        for ligne in &self.tab_defense {
            for element in ligne {
                print!("{} ", element);
            }
            println!();
        }
    }

    pub fn touche(&self) -> bool {
        return true;
    }

    pub fn modifier_case_attaque(&mut self, attaque: String, signe: String) {
        let tableau_attaque = self.string_splitter(attaque);

        //Si pas valide renvoyer E
        if !self.verifier_condition(tableau_attaque[0], tableau_attaque[1]) {
            return;
        }

        let axe_y: usize = ((tableau_attaque[0] as u8) - ('0' as u8)) as usize;
        let axe_x: usize = ((tableau_attaque[1] as u8) - ('A' as u8)) as usize;

        self.tab_attaque[axe_y][axe_x] = signe.chars().next().unwrap();

        return;
    }

    pub fn modifier_case_defense(&self, attaque: String) -> bool {
        return false;
    }

    pub fn poser_bateau(&self, y: char, x: char) -> bool {
        return false;
    }

    pub fn tourner_bateau(&self, rotation: u8) -> bool {
        return false;
    }

    pub fn poser_bateau_aleatoire(&self) {}

    pub fn reinitialiser_tableaux(&self) {}

    pub fn afficher_bateau_vivant(&self) -> u8 {
        return self.nombre_de_bateau_vivant;
    }

    pub fn afficher_bateau_mort(&self) -> u8 {
        return self.nombre_de_bateau_mort;
    }

    // Fonction privée de la classe

    fn verifier_condition(&self, y: char, x: char) -> bool {
        if y >= '0' && y <= '9' && x >= 'A' && x <= 'J' {
            return true;
        }
        return false;
    }

    fn string_splitter(&self, chaine: String) -> [char; 2] {
        let mut tableau = ['\0'; 2]; // Initialise un tableau fixe de taille 5 avec des caractères nuls
        let mut index = 0;

        for c in chaine.chars() {
            if index < tableau.len() {
                tableau[index] = c;
                index += 1;
            }
        }

        return tableau;
    }
}
