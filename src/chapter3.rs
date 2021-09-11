use rltk::{GameState, Rltk, RGB, VirtualKeyCode};
use specs::prelude::*;
use std::cmp::{max, min};
use specs_derive::Component;
use specs::shred::Fetch;

static MAP_SIZE_X: i32 = 80;
static MAP_SIZE_Y: i32 = 50;

impl GameState for State {
    fn tick(&mut self, context: &mut Rltk) {
        context.cls();

        player_input(self, context);
        self.run_systems();

        let map: Fetch<Vec<TileType>> = self.ecs.fetch::<Vec<TileType>>();
        draw_map(&map, context);

        let positions = self.ecs.read_storage::<Position>();
        let renderables = self.ecs.read_storage::<Renderable>();

        for (pos, render) in (&positions, &renderables).join() {
            context.set(pos.x, pos.y, render.foreground, render.background, render.glyph)
        }
    }
}

pub fn run() -> rltk::BError {
    use rltk::RltkBuilder;
    let context = RltkBuilder::simple80x50()
        .with_title("Roguelike Tutorial")
        .build()?;
    let mut gs = State { ecs: World::new() };

    gs.ecs.register::<Position>();
    gs.ecs.register::<Renderable>();
    gs.ecs.register::<Player>();

    gs.ecs
        .create_entity()
        .with(Position { x: 40, y: 25 })
        .with(Renderable {
            glyph: rltk::to_cp437('@'),
            foreground: RGB::named(rltk::YELLOW),
            background: RGB::named(rltk::BLACK),
        })
        .with(Player {})
        .build();

    gs.ecs.insert(new_map());
    rltk::main_loop(context, gs)
}

#[derive(Component)]
struct Position {
    x: i32,
    y: i32,
}

#[derive(Component)]
struct Renderable {
    glyph: rltk::FontCharType,
    foreground: RGB,
    background: RGB,
}

struct State {
    ecs: World,
}

impl State {
    fn run_systems(&mut self) {
        self.ecs.maintain();
    }
}

#[derive(Component, Debug)]
struct Player {}

fn try_move_player(delta_x: i32, delta_y: i32, ecs: &mut World) {
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

fn player_input(gs: &mut State, context: &mut Rltk) {
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

#[derive(PartialEq, Copy, Clone)]
enum TileType {
    Wall,
    Floor,
}

pub fn xy_index(x: i32, y: i32) -> usize {
    (y as usize * MAP_SIZE_X as usize) + x as usize
}

fn new_map() -> Vec<TileType> {
    let mut map = vec![TileType::Floor; (MAP_SIZE_X * MAP_SIZE_Y) as usize];

    // make the walls for the boundaries
    for x in 0..80 {
        map[xy_index(x, 0)] = TileType::Wall;
        map[xy_index(x, MAP_SIZE_Y - 1)] = TileType::Wall;
    }
    for y in 0..50 {
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

//&[TileType] is a slice
/*
Slices are similar to arrays, but their length is not known at compile time. Instead,
a slice is a two-word object, the first word is a pointer to the data, and the
second word is the length of the slice. The word size is the same as usize,
determined by the processor architecture eg 64 bits on an x86-64.
Slices can be used to borrow a section of an array, and have the type signature &[T].
*/
fn draw_map(map: &[TileType], context: &mut Rltk) {
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