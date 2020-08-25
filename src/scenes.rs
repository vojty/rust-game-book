use crate::utils;
use rand::Rng;
use serde::Deserialize;

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
    fn print_header(&self) {
        self.header_print(&self.text)
    }

    fn print_options(&self) {
        for (index, option) in self.options.iter().enumerate() {
            println!("  {}) {}", index + 1, option.text);
        }
    }

    fn get_user_action(&self) -> String {
        loop {
            match utils::read_user_input() {
                Ok(scene_number) => match self.options.get((scene_number - 1) as usize) {
                    Some(selected_option) => return selected_option.scene_id.clone(),
                    None => {
                        println!("Number {} is not an option.", scene_number);
                        continue;
                    }
                },
                Err(_) => {
                    println!("Please enter the number.");
                    continue;
                }
            }
        }
    }

    pub fn play(&self) -> Option<String> {
        self.print_header();
        if self.end {
            return None;
        }
        self.print_options();

        let scene_id = self.get_user_action();
        return Some(scene_id);
    }
}

const SCENE_HP: i32 = 100;
const ENEMY_DMG: i32 = 20;

pub enum BattleResult {
    Win,
    Loss,
    Run,
}

enum AtackResult {
    Damage(i32),
    Run,
    Invalid,
}

impl BattleScene {
    fn print_header(&self) {
        let text = format!("[BATTLE] Wild {} ({} HP) appeared!", self.enemy, self.hp);
        self.header_print(&text);
    }

    fn print_options(&self) {
        println!(" 1) Use your fists");
        println!(" 2) Use shotgun");
        println!(" 3) Use bazooka");
        println!(" 4) RUN");
    }

    fn handle_attack(&self, user_hp: i32, enemy_hp: i32, damage: i32) -> (i32, i32) {
        println!("-> your attack dmg: {}", damage);
        let diff = enemy_hp - damage;

        if diff > 0 {
            // Attack back
            println!("<- enemy attack dmg: {}", damage);
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

    fn battle(&self) -> BattleResult {
        let mut user_hp = SCENE_HP;
        let mut enemy_hp = self.hp;
        loop {
            self.print_options();
            match utils::read_user_input() {
                Ok(user_input) => match self.get_attack_result(user_input) {
                    AtackResult::Damage(damage) => {
                        let (new_user_hp, new_enemy_hp) =
                            self.handle_attack(user_hp, enemy_hp, damage);
                        user_hp = new_user_hp;
                        enemy_hp = new_enemy_hp;
                        if user_hp <= 0 {
                            println!("You died :/");
                            return BattleResult::Loss;
                        }
                        if enemy_hp <= 0 {
                            println!("You killed your enemy, you can go further");
                            return BattleResult::Win;
                        }

                        println!("-- Your HP: {}", user_hp);
                        println!("-- Enemy's HP: {}", enemy_hp);
                    }
                    AtackResult::Run => return BattleResult::Run,
                    AtackResult::Invalid => {
                        println!("Please enter the number (1-3).");
                        continue;
                    }
                },

                Err(_) => {
                    println!("Please enter the number (1-3).");
                    continue;
                }
            }
        }
    }

    pub fn play(&self) -> BattleResult {
        self.print_header();
        return self.battle();
    }
}

pub trait Scene {
    fn get_id(&self) -> &String; // getter
    fn header_print(&self, text: &String) {
        // shared fancy print
        let line: String = (0..text.chars().count() + 2).map(|_| "-").collect();
        println!("+{}+", line);
        println!("| {} |", text);
        println!("+{}+", line);
    }
}

impl Scene for BattleScene {
    fn get_id(&self) -> &String {
        return &self.scene_id;
    }
}

impl Scene for StandardScene {
    fn get_id(&self) -> &String {
        return &self.scene_id;
    }
}
