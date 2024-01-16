/*
Traits allows :
  Interface definition
  Polymorphism
*/
trait Attacker {
    fn choose_style(&self) -> String;
}

#[derive(Debug)]
enum Character {
    Warrior,
    Archer,
    Wizard,
}

impl Attacker for Character {
    fn choose_style(&self) -> String {
        match self {
            Character::Warrior => "wing chun".to_string(),
            Character::Archer => "bow".to_string(),
            Character::Wizard => "magic".to_string(),
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_structs() {
        let my_character: Character = Character::Warrior;
        let chosen_fighting_style: String = my_character.choose_style();
        dbg!(chosen_fighting_style);
    }
}
