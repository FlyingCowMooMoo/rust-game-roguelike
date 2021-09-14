mod game;
mod rect;
mod player;
mod map;
mod components;
mod visibility_system;

fn main() -> rltk::BError {
    game::run()
}