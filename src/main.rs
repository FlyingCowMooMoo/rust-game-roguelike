mod components;
mod game;
mod map;
mod monster_ai_system;
mod player;
mod rect;
mod visibility_system;

fn main() -> rltk::BError {
    game::run()
}
