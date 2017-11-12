use super::Json;
use super::User;
use super::Game;

pub fn get_user(name: &String) -> Json<User> {
    Json(User::new(&name, 1, 1200))
}

pub fn game(gameid: String) -> Json<Game> {
    Json(Game::new(gameid))
}