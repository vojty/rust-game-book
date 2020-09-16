use crate::scenes::BattleScene;
use crate::scenes::Scene;
use crate::scenes::StandardScene;
use serde::de::DeserializeOwned;
use std::collections::HashMap;
use std::fs::File;

pub fn create_game_over() -> String {
    String::from(
        "Game over, here is a panda 🐼 to make you feel better.
If you wanna have some fun, see https://www.youtube.com/watch?v=dQw4w9WgXcQ",
    )
}

pub fn create_game_win() -> String {
    String::from(
        "🍾🍾 You have won! 🍾🍾
https://www.youtube.com/watch?v=GC5E8ie2pdM",
    )
}

pub fn parse_u32(input: String) -> std::result::Result<u32, std::num::ParseIntError> {
    return input.trim().parse::<u32>();
}

const STANDARD_SCENES_PATH: &str = "./assets/standard_scenes.json";
const BATTLE_SCENES_PATH: &str = "./assets/battle_scenes.json";

fn load_json<T: DeserializeOwned>(path: &str) -> Vec<T> {
    let file = File::open(path).expect(&format!("Unable to find {} file.", path));
    let scenes: Vec<T> =
        serde_json::from_reader(file).expect(&format!("Unable to parse JSON from {} file.", path));

    return scenes;
}

// convert to HashMap, so we can easily lookup for the scenes
fn convert_to_hashmap<T: Scene>(data: Vec<T>) -> HashMap<String, T> {
    let mut map = HashMap::new();
    for scene in data {
        map.insert(scene.get_id().clone(), scene);
    }
    map
}

pub fn get_scenes() -> HashMap<String, StandardScene> {
    let data = load_json::<StandardScene>(STANDARD_SCENES_PATH);
    convert_to_hashmap(data)
}

pub fn get_battle_scenes() -> HashMap<String, BattleScene> {
    let data = load_json::<BattleScene>(BATTLE_SCENES_PATH);
    convert_to_hashmap(data)
}
