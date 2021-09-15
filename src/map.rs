use std::cmp::{max, min};

use rltk::{Algorithm2D, BaseMap, Point, RandomNumberGenerator, Rltk, RGB};
use specs::shred::Fetch;
use specs::World;

use crate::rect::Rect;

pub static MAP_SIZE_X: i32 = 80;
pub static MAP_SIZE_Y: i32 = 50;

#[derive(PartialEq, Copy, Clone)]
pub enum TileType {
    Wall,
    Floor,
}

pub struct Map {
    pub tiles: Vec<TileType>,
    pub rooms: Vec<Rect>,
    pub width: i32,
    pub height: i32,
    pub revealed_tiles: Vec<bool>,
    pub visible_tiles: Vec<bool>,
}

impl Algorithm2D for Map {
    fn dimensions(&self) -> Point {
        Point::new(self.width, self.height)
    }
}

impl BaseMap for Map {
    fn is_opaque(&self, index: usize) -> bool {
        self.tiles[index as usize] == TileType::Wall
    }
}

impl Map {
    pub fn xy_index(&self, x: i32, y: i32) -> usize {
        (y as usize * self.width as usize) + x as usize
    }

    fn apply_room_to_map(&mut self, room: &Rect) {
        for y in room.y1 + 1..=room.y2 {
            for x in room.x1 + 1..=room.x2 {
                let index = self.xy_index(x, y);
                self.tiles[index] = TileType::Floor;
            }
        }
    }

    fn apply_horizontal_tunnel(&mut self, x1: i32, x2: i32, y: i32) {
        for x in min(x1, x2)..=max(x1, x2) {
            let index = self.xy_index(x, y);
            if index > 0 && index < (self.width * self.height) as usize {
                self.tiles[index] = TileType::Floor;
            }
        }
    }

    fn apply_vertical_tunnel(&mut self, y1: i32, y2: i32, x: i32) {
        for y in min(y1, y2)..=max(y1, y2) {
            let index = self.xy_index(x, y);
            if index > 0 && index < (self.width * self.height) as usize {
                self.tiles[index] = TileType::Floor;
            }
        }
    }

    pub fn new_map_rooms_and_corridors() -> Map {
        let mut map = Map {
            tiles: vec![TileType::Wall; (MAP_SIZE_X * MAP_SIZE_Y) as usize],
            rooms: Vec::new(),
            width: MAP_SIZE_X,
            height: MAP_SIZE_Y,
            revealed_tiles: vec![false; (MAP_SIZE_X * MAP_SIZE_Y) as usize],
            visible_tiles: vec![false; (MAP_SIZE_X * MAP_SIZE_Y) as usize],
        };

        const MAX_ROOMS: i32 = 30;
        const MIN_SIZE: i32 = 6;
        const MAX_SIZE: i32 = 10;

        let mut rng = RandomNumberGenerator::new();

        for _ in 0..MAX_ROOMS {
            let width = rng.range(MIN_SIZE, MAX_SIZE);
            let height = rng.range(MIN_SIZE, MAX_SIZE);
            let x = rng.roll_dice(1, map.width - width - 1) - 1;
            let y = rng.roll_dice(1, map.height - height - 1) - 1;
            let new_room = Rect::new(x, y, width, height);
            let mut ok = true;
            for other_room in map.rooms.iter() {
                if new_room.intersect(other_room) {
                    ok = false
                }
            }
            if ok {
                map.apply_room_to_map(&new_room);

                if !map.rooms.is_empty() {
                    let (new_x, new_y) = new_room.center();
                    let (prev_x, prev_y) = map.rooms[map.rooms.len() - 1].center();
                    if rng.range(0, 2) == 1 {
                        map.apply_horizontal_tunnel(prev_x, new_x, prev_y);
                        map.apply_vertical_tunnel(prev_y, new_y, new_x);
                    } else {
                        map.apply_horizontal_tunnel(prev_x, new_x, new_y);
                        map.apply_vertical_tunnel(prev_y, new_y, prev_x);
                    }
                }

                map.rooms.push(new_room);
            }
        }
        map
    }
}

pub fn draw_map(ecs: &World, context: &mut Rltk) {
    let map: Fetch<Map> = ecs.fetch::<Map>();

    let mut y = 0;
    let mut x = 0;
    for (index, tile) in map.tiles.iter().enumerate() {
        if map.revealed_tiles[index] {
            let glyph;
            let mut foreground;
            match tile {
                TileType::Floor => {
                    glyph = rltk::to_cp437('.');
                    foreground = RGB::from_f32(0.0, 0.5, 0.5);
                }
                TileType::Wall => {
                    glyph = rltk::to_cp437('#');
                    foreground = RGB::from_f32(0., 1.0, 0.);
                }
            }
            if !map.visible_tiles[index] {
                foreground = foreground.to_greyscale();
            }
            context.set(x, y, foreground, RGB::from_f32(0., 0., 0.), glyph);
        }
        //move the co-ords
        x += 1;
        if x > MAP_SIZE_X - 1 {
            x = 0;
            y += 1;
        }
    }
}