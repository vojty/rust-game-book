use std::{net::Shutdown, sync::Arc};
use tokio::io::AsyncReadExt;
use tokio::io::{self};

use scenes::{BattleResult, BattleScene, Scene, StandardScene};
use tokio::io::AsyncWriteExt;
use tokio::net::TcpListener;
use tokio::net::TcpStream;

mod scenes;
mod utils;

async fn read_from_socket(socket: &mut TcpStream) -> String {
    let mut buffer = vec![0; 1024];
    match socket.read(&mut buffer).await {
        Ok(0) => return String::from(""),
        Ok(n) => {
            return String::from_utf8_lossy(&buffer[0..n]).trim().to_string();
        }
        Err(_) => {
            println!("Reading from socket failed");
            return String::from("");
        }
    }
}

async fn write_to_socket(socket: &mut TcpStream, string: &String) {
    socket.write_all(string.as_bytes()).await.unwrap();
}

async fn play_standard_scene(socket: &mut TcpStream, current_scene: &StandardScene) -> String {
    let header = current_scene.create_header();
    write_to_socket(socket, &header).await;

    let options = current_scene.create_options();
    write_to_socket(socket, &options).await;

    // Input loop
    loop {
        let input = read_from_socket(socket).await;
        match current_scene.play(input) {
            Ok(scene_id) => return scene_id,
            Err(error_message) => {
                write_to_socket(socket, &error_message).await;
            }
        }
    }
}

enum BattleSceneResult {
    Win,
    Loss,
    Run,
}

async fn play_battle_scene(
    socket: &mut TcpStream,
    battle_scene: &BattleScene,
) -> BattleSceneResult {
    let header = battle_scene.create_header();
    write_to_socket(socket, &header).await;

    let options = battle_scene.create_options();
    write_to_socket(socket, &options).await;

    let mut current_state = battle_scene.get_initial_state();

    loop {
        let input = read_from_socket(socket).await;
        match battle_scene.play(input, current_state) {
            BattleResult::Win => {
                let message = String::from("You killed your enemy, you can go further\n");
                write_to_socket(socket, &message).await;
                return BattleSceneResult::Win;
            }
            BattleResult::Continue(new_state) => {
                current_state = new_state;
                let message = String::from(format!(
                    "You: {} HP, enemy: {} HP",
                    current_state.0, current_state.1
                ));
                write_to_socket(socket, &message).await;
            }
            BattleResult::Loss => {
                let message = String::from("You have died fighting :/\n");
                write_to_socket(socket, &message).await;
                return BattleSceneResult::Loss;
            }
            BattleResult::Run => {
                let message = String::from(
                    "You run away like a little girl, so you have to start over again.\n",
                );
                write_to_socket(socket, &message).await;
                return BattleSceneResult::Run;
            }
            BattleResult::InvalidInput => {
                let message = String::from("Invalid input\n");
                write_to_socket(socket, &message).await;
            }
        }
    }
}

#[tokio::main]
async fn main() -> io::Result<()> {
    let mut listener = TcpListener::bind("127.0.0.1:5000").await?;

    let scenes = Arc::new(utils::get_scenes());
    let battle_scenes = Arc::new(utils::get_battle_scenes());

    loop {
        let (mut socket, _) = listener.accept().await?;
        println!("Client connected");

        let scenes = scenes.clone();
        let battle_scenes = battle_scenes.clone();
        tokio::spawn(async move {
            let mut current_scene = scenes.get("coast").unwrap();
            // Game loop
            loop {
                match battle_scenes.get(&current_scene.scene_id) {
                    Some(battle_scene) => {
                        match play_battle_scene(&mut socket, battle_scene).await {
                            BattleSceneResult::Run => {
                                current_scene = scenes.get("coast").unwrap();
                            }
                            BattleSceneResult::Win => {} // noop
                            BattleSceneResult::Loss => break,
                        }
                    }
                    None => (), // current scene has no battle scene prerequisition -> noop
                }

                let new_scene_id = play_standard_scene(&mut socket, current_scene).await;
                current_scene = scenes.get(&new_scene_id).unwrap();

                if current_scene.end {
                    let header = current_scene.create_header();
                    write_to_socket(&mut socket, &header).await;
                    break;
                }
            }

            let message = if current_scene.win {
                utils::create_game_win()
            } else {
                utils::create_game_over()
            };
            write_to_socket(&mut socket, &message).await;
            socket.shutdown(Shutdown::Both).unwrap();
            println!("Client disconnected");
        });
    }
}
