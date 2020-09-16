use rand::Rng;
use serde::Deserialize;

use crate::utils;

#[derive(Deserialize)]
pub struct StandardSceneOption {
    pub scene_id: String,
    pub text: String,
}
#[derive(Deserialize)]
pub struct StandardScene {
    pub scene_id: String,
    pub text: String,
    pub end: bool,
    pub win: bool,
    pub options: Vec<StandardSceneOption>,
}

#[derive(Deserialize)]
pub struct BattleScene {
    pub scene_id: String,
    pub enemy: String,
    pub hp: i32,
}

impl StandardScene {
    fn validate_user_action(&self, input: String) -> Result<String, String> {
        match utils::parse_u32(input) {
            Ok(scene_number) => match self.options.get((scene_number - 1) as usize) {
                Some(selected_option) => Ok(selected_option.scene_id.clone()),
                None => Err(format!("Number {} is not an option.", scene_number)),
            },
            Err(_) => Err(String::from("Please enter the number.")),
        }
    }

    pub fn play(&self, input: String) -> Result<String, String> {
        return self.validate_user_action(input);
    }
}

const SCENE_HP: i32 = 100;
const ENEMY_DMG: i32 = 20;

pub enum BattleResult {
    Win,
    Continue(BattleSceneState),
    Loss,
    Run,
    InvalidInput,
}

enum AtackResult {
    Damage(i32),
    Run,
    Invalid,
}

pub type BattleSceneState = (i32, i32); // user, enemy

impl BattleScene {
    fn handle_attack(&self, current_state: BattleSceneState, damage: i32) -> BattleSceneState {
        let (user_hp, enemy_hp) = current_state;
        let diff = enemy_hp - damage;

        if diff > 0 {
            // Attack back
            return (user_hp - ENEMY_DMG, diff);
        }
        return (user_hp, 0);
    }

    fn get_attack_result(&self, user_input: u32) -> AtackResult {
        let mut rng = rand::thread_rng();
        match user_input {
            1 => return AtackResult::Damage(rng.gen_range(5, 10)),
            2 => return AtackResult::Damage(20),
            3 => return AtackResult::Damage(100),
            4 => return AtackResult::Run,
            _ => return AtackResult::Invalid,
        }
    }

    fn battle(&self, input: u32, current_state: BattleSceneState) -> BattleResult {
        match self.get_attack_result(input) {
            AtackResult::Damage(damage) => {
                let (user_hp, enemy_hp) = self.handle_attack(current_state, damage);
                if user_hp <= 0 {
                    return BattleResult::Loss;
                }
                if enemy_hp <= 0 {
                    return BattleResult::Win;
                }
                return BattleResult::Continue((user_hp, enemy_hp));
            }
            AtackResult::Run => return BattleResult::Run,
            AtackResult::Invalid => BattleResult::InvalidInput,
        }
    }

    pub fn get_initial_state(&self) -> BattleSceneState {
        (SCENE_HP, self.hp)
    }

    pub fn play(&self, input: String, current_state: BattleSceneState) -> BattleResult {
        match utils::parse_u32(input) {
            Ok(user_input) => return self.battle(user_input, current_state),
            Err(_) => return BattleResult::InvalidInput,
        }
    }
}

pub trait Scene {
    fn get_id(&self) -> &String; // getter
    fn create_header(&self) -> String;
    fn create_options(&self) -> String;
}

impl Scene for BattleScene {
    fn get_id(&self) -> &String {
        return &self.scene_id;
    }
    fn create_header(&self) -> String {
        format!("[BATTLE] Wild {} ({} HP) appeared!\n", self.enemy, self.hp)
    }
    fn create_options(&self) -> String {
        String::from(" 1) Use your fists\n 2) Use shotgun\n 3) Use bazooka\n 4) RUN\n")
    }
}

impl Scene for StandardScene {
    fn get_id(&self) -> &String {
        return &self.scene_id;
    }
    fn create_header(&self) -> String {
        let line: String = (0..self.text.chars().count() + 2).map(|_| "-").collect();
        return String::from(format!("+{}+\n| {} |\n+{}+\n", line, self.text, line));
    }
    fn create_options(&self) -> String {
        let lines: Vec<String> = self
            .options
            .iter()
            .enumerate()
            .map(|(index, option)| format!("  {}) {}", index + 1, option.text))
            .collect();
        lines.join("\n").to_string()
    }
}
