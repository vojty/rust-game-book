use scenes::BattleResult;

mod scenes;
mod utils;

fn game_loop() {
    let battle_scenes = utils::get_battle_scenes();
    let scenes = utils::get_scenes();

    let mut current_scene = scenes.get("coast").expect("There is no first scene.");
    loop {
        match battle_scenes.get(&current_scene.scene_id) {
            Some(battle_scene) => {
                match battle_scene.play() {
                    BattleResult::Win => (), // noop
                    BattleResult::Loss => break,
                    BattleResult::Run => {
                        println!(
                            "You run away like a little girl, so you have to start over again."
                        );
                        current_scene = scenes.get("coast").expect("There is no first scene.");
                        continue;
                    }
                }
            }
            None => (), // current scene has no battle scene prerequisition -> noop
        }

        match current_scene.play() {
            Some(scene_id) => {
                current_scene = scenes.get(&scene_id).expect("This should never happen :)")
            }
            None => break,
        }
    }

    if current_scene.win {
        utils::print_game_win();
    } else {
        utils::print_game_over();
    }
}

fn main() {
    game_loop();
}
