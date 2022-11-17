
pub struct Allergies {
    aller: Vec<Allergen>,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Allergen {
    Eggs = 1,
    Peanuts = 2,
    Shellfish = 4,
    Strawberries = 8,
    Tomatoes = 16,
    Chocolate = 32,
    Pollen = 64,
    Cats = 128,
}

impl Allergies {
    pub fn new(score: u32) -> Self {
        let mut all_ver: Vec<Allergen> = vec![];
        let mut new_score = score % 256;
        for i in (0..8).rev() {
            println!("AAAA");
            if new_score >= 2_u32.pow(i) {
                println!("{}, {:?}", i, all_ver);
                new_score -= 2_u32.pow(i);
                all_ver.push(match 2_u32.pow(i) {
                    128 => Allergen::Cats,
                    64 => Allergen::Pollen,
                    32 => Allergen::Chocolate,
                    16 => Allergen::Tomatoes,
                    8 => Allergen::Strawberries,
                    4 => Allergen::Shellfish,
                    2 => Allergen::Peanuts,
                    1 => Allergen::Eggs,
                    _ => panic!(),
                })
            } 
        }
        println!("{:?}", all_ver);
        return Self {
            aller : all_ver,
        }
    }

    pub fn is_allergic_to(&self, allergen: &Allergen) -> bool {
        self.aller.iter().find(|&a| a == allergen) != None
        //return ((allergen < Allergen::Cats) && (allergen.sqrt().fract() = 0.0) )
    }

    pub fn allergies(&self) -> Vec<Allergen> {
        self.aller.clone()
    }
}

pub fn main() {
    assert!(Allergies::new(5).is_allergic_to(&Allergen::Cats));
    //println!("{:?}", a)
}
