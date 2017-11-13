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
use functions::User;
use functions::Move;
use functions::Game;

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

//Configuration
fn main() {
    rocket::ignite()
        .mount("/", routes![index])
        .mount("/u", routes![get_user_for_name])
        .mount("/g", routes![get_game, add_move_to_game])
        .launch();
}
