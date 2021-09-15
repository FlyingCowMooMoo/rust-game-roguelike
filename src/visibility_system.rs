use rltk::{field_of_view, Point};
use specs::prelude::*;

use crate::components::{Player, Position, Viewshed};
use crate::map::Map;

pub struct VisibilitySystem {}

impl<'life_time_a> System<'life_time_a> for VisibilitySystem {
    type SystemData = (
        WriteExpect<'life_time_a, Map>,
        Entities<'life_time_a>,
        WriteStorage<'life_time_a, Viewshed>,
        WriteStorage<'life_time_a, Position>,
        ReadStorage<'life_time_a, Player>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (mut map, entities, mut viewshed, position, player): (
            WriteExpect<Map>,
            Entities,
            WriteStorage<Viewshed>,
            WriteStorage<Position>,
            ReadStorage<Player>,
        ) = data;

        for (entity, viewshed, position) in (&entities, &mut viewshed, &position).join() {
            if viewshed.dirty {
                viewshed.visible_tiles.clear();
                viewshed.visible_tiles =
                    field_of_view(Point::new(position.x, position.y), viewshed.range, &*map);
                viewshed.visible_tiles.retain(|pos| {
                    pos.x >= 0 && pos.x < map.width && pos.y >= 0 && pos.y < map.height
                });

                player.get(entity).map(|_| {
                    for it in map.visible_tiles.iter_mut() {
                        *it = false
                    }
                    for vis in viewshed.visible_tiles.iter() {
                        let index = map.xy_index(vis.x, vis.y);
                        map.revealed_tiles[index] = true;
                        map.visible_tiles[index] = true;
                    }
                });
            }
        }
    }
}
