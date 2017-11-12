#![feature(plugin)]
#![plugin(rocket_codegen)]

mod functions;

extern crate rocket;
extern crate rocket_contrib;
#[macro_use]
extern crate serde_derive;

use rocket_contrib::Json;
use functions::game;
use functions::get_user;

//  Structs

#[derive(Default, Debug, Serialize, Deserialize)]
struct Move {
    x1: u16,
    y1: u16,
    x2: u16,
    y2: u16,
    chessman: String
}

#[derive(Default, Debug, Serialize, Deserialize)]
struct Game {
    id: String,
    moves: Vec<Move>
}
impl Game {
    fn new(ident: String) -> Game {
        Game {
            id: ident,
            moves: Vec::new()
        }
    }
    fn add_move(&mut self, m: Move) {
        self.moves.push(m);
    }
}

#[derive(Default, Debug, Serialize, Deserialize)]
struct User {
    rank: u64,
    points: u64,
    name: String
}
impl User {
    fn new(n: &String, r: u64, p: u64) -> User {
        User {
            name: n.to_ascii_lowercase(),
            rank: r,
            points: p
        }
    }
}

//  Access-points
#[get("/")]
fn index() -> String {
    "Index".to_ascii_uppercase()
}

#[get("/<name>")]
fn get_user_for_name(name: String) -> Json<User> {
    get_user(&name)    
}

#[get("/<gameid>")]
fn get_game(gameid: String) -> Json<Game> {
    game(gameid)
}

#[post("/<gameid>/move", format = "application/json", data = "<chessmove>")]
fn add_move_to_game(gameid: String, chessmove: Json<Move>) -> Json<Game> {
    let mut g: Game = game(gameid).into_inner();
    g.add_move(chessmove.into_inner());
    Json(g)
}

fn main() {
    rocket::ignite()
        .mount("/", routes![index])
        .mount("/u", routes![get_user_for_name])
        .mount("/g", routes![get_game, add_move_to_game])
        .launch();
}

/*
#[post("/addGame", format = "application/json", data = "<game>")]
fn addGame(game: Json<Game>) {

}*/

