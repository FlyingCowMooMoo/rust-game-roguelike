use rltk::{GameState, Rltk, RGB};
use specs::prelude::*;
use specs::shred::Fetch;
use crate::chapter4::map::new_map;
use crate::chapter4::map::TileType;
use crate::chapter4::map::MAP_SIZE_X;
use crate::chapter4::map::MAP_SIZE_Y;
use crate::chapter4::map::draw_map;
use crate::chapter4::player::player_input;
use crate::chapter4::components::Player;
use crate::chapter4::components::Position;
use crate::chapter4::components::Renderable;


pub struct State {
    pub ecs: World,
}

impl State {
    fn run_systems(&mut self) {
        self.ecs.maintain();
    }
}

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
        .with(Position { x: MAP_SIZE_X / 2, y: MAP_SIZE_Y / 2 })
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