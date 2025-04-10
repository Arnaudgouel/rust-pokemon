#[derive(Debug, Clone, PartialEq)]
enum Type {
    Feu,
    Eau,
    Plante,
    Electrik,
    Roche,
    Psy,
    Vol,
    Insecte,
    Normal,
    Combat,
    Poison,
    Spectre,
    Dragon,
    Glace,
}

#[derive(Debug, Clone, PartialEq)]
enum Genre {
    Male,
    Femelle,
}

#[derive(Debug, Clone)]
struct Pokemon {
    nom: String,
    niveau: u8,
    type_: Type,
    experience: u32,
    genre: Genre,
    evolution: Option<String>,
}

fn generer_pokemons() -> Vec<Pokemon> {
  vec![
      Pokemon { nom: "Pikachu".to_string(), niveau: 12, type_: Type::Electrik, experience: 230, genre: Genre::Male, evolution: Some("Raichu".to_string()) },
      Pokemon { nom: "Salamèche".to_string(), niveau: 10, type_: Type::Feu, experience: 160, genre: Genre::Femelle, evolution: Some("Reptincel".to_string()) },
      Pokemon { nom: "Carapuce".to_string(), niveau: 10, type_: Type::Eau, experience: 180, genre: Genre::Male, evolution: Some("Carabaffe".to_string()) },
      Pokemon { nom: "Bulbizarre".to_string(), niveau: 9, type_: Type::Plante, experience: 120, genre: Genre::Femelle, evolution: Some("Herbizarre".to_string()) },
      Pokemon { nom: "Rondoudou".to_string(), niveau: 6, type_: Type::Normal, experience: 80, genre: Genre::Femelle, evolution: Some("Grodoudou".to_string()) },
      Pokemon { nom: "Nosferapti".to_string(), niveau: 7, type_: Type::Poison, experience: 90, genre: Genre::Male, evolution: Some("Nosferalto".to_string()) },
      Pokemon { nom: "Caninos".to_string(), niveau: 11, type_: Type::Feu, experience: 210, genre: Genre::Male, evolution: Some("Arcanin".to_string()) },
      Pokemon { nom: "Magicarpe".to_string(), niveau: 5, type_: Type::Eau, experience: 60, genre: Genre::Femelle, evolution: Some("Léviator".to_string()) },
      Pokemon { nom: "Racaillou".to_string(), niveau: 13, type_: Type::Roche, experience: 270, genre: Genre::Male, evolution: Some("Gravalanch".to_string()) },
      Pokemon { nom: "Psykokwak".to_string(), niveau: 9, type_: Type::Psy, experience: 150, genre: Genre::Femelle, evolution: Some("Akwakwak".to_string()) },
      Pokemon { nom: "Roucool".to_string(), niveau: 8, type_: Type::Vol, experience: 130, genre: Genre::Male, evolution: Some("Roucoups".to_string()) },
      Pokemon { nom: "Chenipan".to_string(), niveau: 3, type_: Type::Insecte, experience: 20, genre: Genre::Femelle, evolution: Some("Chrysacier".to_string()) },
      Pokemon { nom: "Machoc".to_string(), niveau: 14, type_: Type::Combat, experience: 290, genre: Genre::Male, evolution: Some("Machopeur".to_string()) },
      Pokemon { nom: "Spectrum".to_string(), niveau: 16, type_: Type::Spectre, experience: 320, genre: Genre::Femelle, evolution: Some("Ectoplasma".to_string()) },
      Pokemon { nom: "Dracolosse".to_string(), niveau: 45, type_: Type::Dragon, experience: 800, genre: Genre::Male, evolution: None }, // Dernière forme
      Pokemon { nom: "Lokhlass".to_string(), niveau: 30, type_: Type::Glace, experience: 600, genre: Genre::Femelle, evolution: None }, // Pas d'évolution
      Pokemon { nom: "Évoli".to_string(), niveau: 6, type_: Type::Normal, experience: 75, genre: Genre::Femelle, evolution: Some("Aquali/Voltali/Pyroli".to_string()) }, // à choisir plus tard
      Pokemon { nom: "Miaouss".to_string(), niveau: 7, type_: Type::Normal, experience: 85, genre: Genre::Male, evolution: Some("Persian".to_string()) },
      Pokemon { nom: "Voltorbe".to_string(), niveau: 9, type_: Type::Electrik, experience: 110, genre: Genre::Femelle, evolution: Some("Électrode".to_string()) },
      Pokemon { nom: "Akwakwak".to_string(), niveau: 18, type_: Type::Eau, experience: 350, genre: Genre::Male, evolution: None },
  ]
}

impl Pokemon {
  pub fn gagner_experience(&mut self, xp: u32) {
      self.experience += xp;
      let niveaux_gagnes = (self.experience / 100) as u8;
      if niveaux_gagnes > self.niveau {
          self.niveau = niveaux_gagnes;
      }
  }

  pub fn afficher(&self) {
      println!("Pokemon: {}", self.nom);
      println!("Niveau: {}", self.niveau);
      println!("Type: {:?}", self.type_);
      println!("Experience: {}", self.experience);
      println!("Genre: {:?}", self.genre);
      println!("Evolution: {:?}", self.evolution);
  }

  pub fn peut_reproduire(&self, autre: &Pokemon) -> bool {
      const MIN_LEVEL: u8 = 10;
      self.type_ == autre.type_ && 
      self.niveau >= MIN_LEVEL && 
      autre.niveau >= MIN_LEVEL && 
      self.genre != autre.genre
  }

  pub fn nouveau_bebe(parent1: &Pokemon, parent2: &Pokemon) -> Option<Pokemon> {
      if !parent1.peut_reproduire(parent2) {
          return None;
      }

      Some(Pokemon {
          nom: "Mystere".to_string(),
          niveau: 1,
          type_: parent1.type_.clone(),
          experience: 0,
          genre: if rand::random() {
              Genre::Male
          } else {
              Genre::Femelle
          },
          evolution: None,
      })
  }
}

// Add this after the Pokemon implementation

use std::collections::HashMap;

#[derive(Debug)]
struct Elevage {
    pokemons: HashMap<String, Pokemon>,
}

impl Elevage {
    pub fn new() -> Self {
        Elevage {
            pokemons: HashMap::new(),
        }
    }

    pub fn ajouter_pokemon(&mut self, pokemon: Pokemon) {
        self.pokemons.insert(pokemon.nom.clone(), pokemon);
    }

    pub fn afficher_tous(&self) {
        for pokemon in self.pokemons.values() {
            pokemon.afficher();
            println!("------------------");
        }
    }

    pub fn entrainer_tous(&mut self, xp: u32) {
        for pokemon in self.pokemons.values_mut() {
            pokemon.gagner_experience(xp);
        }
    }

    pub fn tenter_reproduction(&mut self, nom1: &str, nom2: &str) -> Option<Pokemon> {
        let parent1 = self.pokemons.get(nom1)?;
        let parent2 = self.pokemons.get(nom2)?;
        
        Pokemon::nouveau_bebe(parent1, parent2)
    }

    pub fn trier_par_niveau(&self) -> Vec<&Pokemon> {
        let mut pokemons: Vec<&Pokemon> = self.pokemons.values().collect();
        pokemons.sort_by_key(|p| p.niveau);
        pokemons
    }

    pub fn trier_par_type(&self) -> Vec<&Pokemon> {
      let mut pokemons: Vec<&Pokemon> = self.pokemons.values().collect();
      pokemons.sort_by_key(|p| match p.type_ {
          Type::Feu => 0,
          Type::Eau => 1,
          Type::Plante => 2,
          Type::Electrik => 3,
          Type::Roche => 4,
          Type::Psy => 5,
          Type::Vol => 6,
          Type::Insecte => 7,
          Type::Normal => 8,
          Type::Combat => 9,
          Type::Poison => 10,
          Type::Spectre => 11,
          Type::Dragon => 12,
          Type::Glace => 13,
      });
      pokemons
  }
}

fn main() {
  let mut elevage = Elevage::new();
  
  // Ajouter quelques Pokemon depuis la fonction generer_pokemons
  for pokemon in generer_pokemons() {
      elevage.ajouter_pokemon(pokemon);
  }

  println!("=== Tous les Pokemon ===");
  elevage.afficher_tous();

  println!("\n=== Entrainement ===");
  elevage.entrainer_tous(50);
  
  println!("\n=== Tentative de reproduction ===");
  if let Some(bebe) = elevage.tenter_reproduction("Caninos", "Salamèche") {
      println!("Nouveau Pokemon né !");
      bebe.afficher();
  } else {
      println!("La reproduction a échoué.");
  }
  if let Some(bebe) = elevage.tenter_reproduction("Caninos", "Bulbizarre") {
      println!("Nouveau Pokemon né !");
      bebe.afficher();
  } else {
      println!("La reproduction a échoué.");
  }

  println!("\n=== Pokemon triés par niveau ===");
  for pokemon in elevage.trier_par_niveau() {
      println!("{}: niveau {}", pokemon.nom, pokemon.niveau);
  }
}


