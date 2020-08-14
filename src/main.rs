use serde::Deserialize;
use std::collections::HashMap;
use std::fs::File;
use std::io;

#[derive(Deserialize)]
struct SceneOption {
    scene_id: String,
    text: String,
}
#[derive(Deserialize)]
struct Scene {
    scene_id: String,
    text: String,
    end: bool,
    win: bool,
    options: Vec<SceneOption>,
}

impl Scene {
    fn print_header(&self) {
        let line: String = (0..self.text.len() + 2).map(|_| "-").collect();
        println!("+{}+", line);
        println!("| {} |", self.text);
        println!("+{}+", line);
    }

    fn print_options(&self) {
        for (index, option) in self.options.iter().enumerate() {
            println!("  {}) {}", index + 1, option.text);
        }
    }
}

const FILE_PATH: &str = "./src/game.json";

fn print_game_over() {
    println!("Game over, here is a panda 🐼 to make you feel better.");
    println!("If you wanna have some fun, see https://www.youtube.com/watch?v=dQw4w9WgXcQ");
}

fn print_game_win() {
    println!("🍾🍾 You have won! 🍾🍾");
    println!("https://www.youtube.com/watch?v=GC5E8ie2pdM")
}

fn get_scenes() -> HashMap<String, Scene> {
    let game_file = File::open(FILE_PATH).expect("Unable to find game.json file.");
    let scenes: Vec<Scene> =
        serde_json::from_reader(game_file).expect("Unable to parse JSON file.");

    // convert to HashMap, so we can easily lookup for the scenes
    let mut map = HashMap::new();
    for scene in scenes {
        map.insert(scene.scene_id.clone(), scene);
    }
    map
}

fn get_user_action(options: &Vec<SceneOption>) -> String {
    loop {
        let mut action: String = String::new();
        io::stdin().read_line(&mut action).unwrap();

        let result = action.trim().parse::<u32>();
        match result {
            Ok(scene_number) => match options.get((scene_number - 1) as usize) {
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

fn game_loop() {
    let scenes = get_scenes();

    let mut current_scene = scenes.get("coast").expect("There is no first scene.");

    loop {
        current_scene.print_header();
        if current_scene.end {
            break;
        }
        current_scene.print_options();

        let scene_id = get_user_action(&current_scene.options);
        current_scene = scenes.get(&scene_id).expect("This should never happen :)");
    }

    if current_scene.win {
        print_game_win();
    } else {
        print_game_over();
    }
}

fn main() {
    game_loop();
}
