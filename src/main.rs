mod game;
mod rect;
mod player;
mod map;
mod components;

fn main() -> rltk::BError {
    game::run()
}