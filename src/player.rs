use std::cmp::{max, min};

use rltk::{Point, Rltk, VirtualKeyCode};
use specs::prelude::*;

use crate::components::{Player, Position, Viewshed};
use crate::game::{RunState, State};
use crate::map::{Map, TileType, MAP_SIZE_X, MAP_SIZE_Y};

pub fn try_move_player(delta_x: i32, delta_y: i32, ecs: &mut World) {
    let mut positions = ecs.write_storage::<Position>();
    let mut players = ecs.write_storage::<Player>();
    let mut viewsheds = ecs.write_storage::<Viewshed>();
    let mut player_position = ecs.write_resource::<Point>();
    let map = ecs.fetch::<Map>();

    for (_player, position, viewshed) in (&mut players, &mut positions, &mut viewsheds).join() {
        let destination_index = map.xy_index(position.x + delta_x, position.y + delta_y);
        if map.tiles[destination_index] != TileType::Wall {
            position.x = min(MAP_SIZE_X - 1, max(0, position.x + delta_x));
            position.y = min(MAP_SIZE_Y - 1, max(0, position.y + delta_y));

            player_position.x = position.x;
            player_position.y = position.x;

            viewshed.dirty = true;
        }
    }
}

pub fn player_input(gs: &mut State, context: &mut Rltk) -> RunState {
    match context.key {
        None => return RunState::Paused, // Nothing happened here
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

            _ => return RunState::Paused,
        },
    }
    RunState::Running
}
