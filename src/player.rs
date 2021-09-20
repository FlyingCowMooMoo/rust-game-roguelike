use std::cmp::{max, min};

use rltk::{Point, Rltk, VirtualKeyCode};
use specs::prelude::*;

use crate::components::{CombatStats, Player, Position, Viewshed, WantsToMelee};
use crate::game::{RunState, State};
use crate::map::{Map, MAP_SIZE_X, MAP_SIZE_Y};

pub fn try_move_player(delta_x: i32, delta_y: i32, ecs: &mut World) {
    let mut positions = ecs.write_storage::<Position>();
    let mut players = ecs.write_storage::<Player>();
    let mut viewsheds = ecs.write_storage::<Viewshed>();
    let mut player_position = ecs.write_resource::<Point>();
    let map = ecs.fetch::<Map>();
    let combat_stats = ecs.read_storage::<CombatStats>();
    let entities = ecs.entities();
    let mut wants_to_melee = ecs.write_storage::<WantsToMelee>();

    for (entity, _player, position, viewshed) in
        (&entities, &mut players, &mut positions, &mut viewsheds).join()
    {
        if position.x + delta_x < 1
            || position.x + delta_x > map.width - 1
            || position.y + delta_y < 1
            || position.y + delta_y > map.height - 1
        {
            return;
        }
        let destination_index = map.xy_index(position.x + delta_x, position.y + delta_y);
        for potential_target in map.tile_content[destination_index].iter() {
            combat_stats.get(*potential_target).map(|_| {
                wants_to_melee
                    .insert(
                        entity,
                        WantsToMelee {
                            target: *potential_target,
                        },
                    )
                    .expect("Add target failed");
                return; // So we don't move after attacking
            });
        }
        if !map.blocked[destination_index] {
            position.x = min(MAP_SIZE_X - 1, max(0, position.x + delta_x));
            position.y = min(MAP_SIZE_Y - 1, max(0, position.y + delta_y));

            player_position.x = position.x;
            player_position.y = position.y;

            viewshed.dirty = true;
        }
    }
}

pub fn player_input(gs: &mut State, context: &mut Rltk) -> RunState {
    match context.key {
        None => return RunState::AwaitingInput, // Nothing happened here
        Some(key) => match key {
            VirtualKeyCode::Left | VirtualKeyCode::Numpad4 | VirtualKeyCode::A => {
                try_move_player(-1, 0, &mut gs.ecs)
            }

            VirtualKeyCode::Right | VirtualKeyCode::Numpad6 | VirtualKeyCode::D => {
                try_move_player(1, 0, &mut gs.ecs)
            }

            VirtualKeyCode::Up | VirtualKeyCode::Numpad8 | VirtualKeyCode::W => {
                try_move_player(0, -1, &mut gs.ecs)
            }

            VirtualKeyCode::Down | VirtualKeyCode::Numpad2 | VirtualKeyCode::S => {
                try_move_player(0, 1, &mut gs.ecs)
            }

            VirtualKeyCode::Numpad9 | VirtualKeyCode::E => try_move_player(1, -1, &mut gs.ecs),

            VirtualKeyCode::Numpad7 | VirtualKeyCode::Q => try_move_player(-1, -1, &mut gs.ecs),

            VirtualKeyCode::Numpad3 | VirtualKeyCode::X => try_move_player(1, 1, &mut gs.ecs),

            VirtualKeyCode::Numpad1 | VirtualKeyCode::Z => try_move_player(-1, 1, &mut gs.ecs),

            _ => return RunState::AwaitingInput,
        },
    }
    RunState::PlayerTurn
}
