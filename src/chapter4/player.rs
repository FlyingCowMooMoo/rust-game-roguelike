use crate::chapter4::map::TileType;
use specs::prelude::*;
use crate::chapter4::map::xy_index;
use crate::chapter4::map::MAP_SIZE_X;
use crate::chapter4::map::MAP_SIZE_Y;
use crate::chapter4::components::Player;
use crate::chapter4::components::Position;
use crate::chapter4::chapter4::State;
use std::cmp::{max, min};
use rltk::{Rltk, VirtualKeyCode};

pub fn try_move_player(delta_x: i32, delta_y: i32, ecs: &mut World) {
    let mut positions = ecs.write_storage::<Position>();
    let mut players = ecs.write_storage::<Player>();
    let map = ecs.fetch::<Vec<TileType>>();

    for (_player, pos) in (&mut players, &mut positions).join() {
        let destination_index = xy_index(pos.x + delta_x, pos.y + delta_y);
        if map[destination_index] != TileType::Wall {
            pos.x = min(MAP_SIZE_X - 1, max(0, pos.x + delta_x));
            pos.y = min(MAP_SIZE_Y - 1, max(0, pos.y + delta_y))
        }
    }
}

pub fn player_input(gs: &mut State, context: &mut Rltk) {
    match context.key {
        None => {} // Nothing happened here
        Some(key) => match key {
            VirtualKeyCode::Left => try_move_player(-1, 0, &mut gs.ecs),
            VirtualKeyCode::Right => try_move_player(1, 0, &mut gs.ecs),
            VirtualKeyCode::Up => try_move_player(0, -1, &mut gs.ecs),
            VirtualKeyCode::Down => try_move_player(0, 1, &mut gs.ecs),
            _ => {}
        }
    }
}