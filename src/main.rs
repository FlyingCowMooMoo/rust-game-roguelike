mod components;
mod damage_system;
mod game;
mod map;
mod map_indexing_system;
mod melee_combat_system;
mod monster_ai_system;
mod player;
mod rect;
mod visibility_system;

fn main() -> rltk::BError {
    game::run()
}
