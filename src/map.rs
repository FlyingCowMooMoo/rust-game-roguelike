use rltk::{Rltk, RGB, RandomNumberGenerator};
use std::cmp::{min, max};
use crate::rect::Rect;

pub static MAP_SIZE_X: i32 = 80;
pub static MAP_SIZE_Y: i32 = 50;
pub static MAP_SIZE: i32 = MAP_SIZE_X * MAP_SIZE_Y;

#[derive(PartialEq, Copy, Clone)]
pub enum TileType {
    Wall,
    Floor,
}


pub fn xy_index(x: i32, y: i32) -> usize {
    (y as usize * MAP_SIZE_X as usize) + x as usize
}



pub fn new_map_rooms_and_corridors() -> (Vec<Rect>, Vec<TileType>) {
    let mut map = vec![TileType::Wall; (MAP_SIZE) as usize];

    let mut rooms: Vec<Rect> = Vec::new();
    const MAX_ROOMS: i32 = 30;
    const MIN_SIZE: i32 = 6;
    const MAX_SIZE: i32 = 10;

    let mut rng = RandomNumberGenerator::new();

    for _ in 0..MAX_ROOMS {
        let width = rng.range(MIN_SIZE, MAX_SIZE);
        let height = rng.range(MIN_SIZE, MAX_SIZE);
        let x = rng.roll_dice(1, MAP_SIZE_X - width - 1) - 1;
        let y = rng.roll_dice(1, MAP_SIZE_Y - height - 1) - 1;
        let new_room = Rect::new(x, y, width, height);
        let mut ok = true;
        for other_room in rooms.iter() {
            if new_room.intersect(other_room) { ok = false }
        }
        if ok {
            apply_room_to_map(&new_room, &mut map);

            if !rooms.is_empty() {
                let (new_x, new_y) = new_room.center();
                let (prev_x, prev_y) = rooms[rooms.len() - 1].center();
                if rng.range(0, 2) == 1  {
                    apply_horizontal_tunnel(&mut map, prev_x, new_x, prev_y);
                    apply_vertical_tunnel(&mut map, prev_y, new_y, new_x);
                } else {
                    apply_horizontal_tunnel(&mut map, prev_x, new_x, new_y);
                    apply_vertical_tunnel(&mut map, prev_y, new_y, prev_x);
                }
            }

            rooms.push(new_room);
        }
    }
    (rooms, map)
}

fn apply_room_to_map(room : &Rect, map: &mut [TileType]) {
    for y in room.y1 +1 ..= room.y2 {
        for x in room.x1 + 1 ..= room.x2 {
            map[xy_index(x, y)] = TileType::Floor;
        }
    }
}

fn apply_horizontal_tunnel(map: &mut [TileType], x1: i32, x2:i32, y: i32) {
    for x in min(x1, x2) ..= max(x1, x2) {
        let index = xy_index(x, y);
        if index > 0 && index < (MAP_SIZE) as usize {
            map[index as usize] = TileType::Floor;
        }
    }
}

fn apply_vertical_tunnel(map: &mut [TileType], y1: i32, y2: i32, x: i32) {
    for y in min(y1, y2) ..= max(y1, y2) {
        let index = xy_index(x, y);
        if index > 0 && index < (MAP_SIZE as usize) {
            map[index as usize] = TileType::Floor;
        }
    }
}

/// Old map method - Making new one in chapter 4
pub fn new_map_test() -> Vec<TileType> {
    let mut map = vec![TileType::Floor; (MAP_SIZE) as usize];

    // make the walls for the boundaries
    for x in 0..MAP_SIZE_X {
        map[xy_index(x, 0)] = TileType::Wall;
        map[xy_index(x, MAP_SIZE_Y - 1)] = TileType::Wall;
    }
    for y in 0..MAP_SIZE_Y {
        map[xy_index(0, y)] = TileType::Wall;
        map[xy_index(MAP_SIZE_X - 1, y)] = TileType::Wall;
    }

    let mut rng = rltk::RandomNumberGenerator::new();
    for _i in 0..400 {
        let x = rng.roll_dice(1, MAP_SIZE_X - 1);
        let y = rng.roll_dice(1, MAP_SIZE_Y - 1);
        let index = xy_index(x, y);
        if index != xy_index(MAP_SIZE_X / 2, MAP_SIZE_Y / 2) {
            map[index] = TileType::Wall
        }
    }

    map
}

pub fn draw_map(map: &[TileType], context: &mut Rltk) {
    let mut x = 0;
    let mut y = 0;
    for tile in map.iter() {
        //Render a tile depending on the type of the tile
        match tile {
            TileType::Floor => {
                context.set(x, y, RGB::from_f32(0.5, 0.5, 0.5),
                            RGB::from_f32(0., 0., 0.),
                            rltk::to_cp437('.'));
            }
            TileType::Wall => {
                context.set(x, y, RGB::from_f32(0.0, 1.0, 0.0),
                            RGB::from_f32(0., 0., 0.),
                            rltk::to_cp437('#'));
            }
        }

        //move the co-ords
        x += 1;
        if x > MAP_SIZE_X - 1 {
            x = 0;
            y += 1;
        }
    }
}