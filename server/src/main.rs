use std::{net::Shutdown, sync::Arc};
use tokio::io::AsyncReadExt;
use tokio::io::{self};

use scenes::Scene;
use tokio::io::AsyncWriteExt;
use tokio::net::TcpListener;
use tokio::net::TcpStream;

mod scenes;
mod utils;

// fn game_loop() {
//     let battle_scenes = utils::get_battle_scenes();
//     let scenes = utils::get_scenes();

//     let mut current_scene = scenes.get("coast").expect("There is no first scene.");
//     loop {
//         match battle_scenes.get(&current_scene.scene_id) {
//             Some(battle_scene) => {
//                 match battle_scene.play() {
//                     BattleResult::Win => (), // noop
//                     BattleResult::Loss => break,
//                     BattleResult::Run => {
//                         println!(
//                             "You run away like a little girl, so you have to start over again."
//                         );
//                         current_scene = scenes.get("coast").expect("There is no first scene.");
//                         continue;
//                     }
//                 }
//             }
//             None => (), // current scene has no battle scene prerequisition -> noop
//         }

//         match current_scene.play() {
//             Some(scene_id) => {
//                 current_scene = scenes.get(&scene_id).expect("This should never happen :)")
//             }
//             None => break,
//         }
//     }

//     if current_scene.win {
//         utils::print_game_win();
//     } else {
//         utils::print_game_over();
//     }
// }

// fn main() {
//     game_loop();
// }

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

#[tokio::main]
async fn main() -> io::Result<()> {
    let mut listener = TcpListener::bind("127.0.0.1:5000").await?;

    let scenes = Arc::new(utils::get_scenes());

    loop {
        let (mut socket, _) = listener.accept().await?;
        println!("Client connected");

        let scenes = scenes.clone();
        tokio::spawn(async move {
            let mut current_scene = scenes.get("coast").unwrap();

            // Game loop
            loop {
                let header = current_scene.create_header();
                write_to_socket(&mut socket, &header).await;

                let options = current_scene.create_options();
                write_to_socket(&mut socket, &options).await;

                // Input loop
                loop {
                    let input = read_from_socket(&mut socket).await;
                    match current_scene.play(input) {
                        Ok(scene_id) => {
                            current_scene = scenes.get(&scene_id).unwrap();
                            break;
                        }
                        Err(error_message) => {
                            write_to_socket(&mut socket, &error_message).await;
                        }
                    }
                }

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
