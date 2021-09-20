use crate::components::{BlocksTile, Position};
use crate::map::Map;
use specs::prelude::*;

pub struct MapIndexingSystem {}

impl<'a> System<'a> for MapIndexingSystem {
    type SystemData = (
        WriteExpect<'a, Map>,
        ReadStorage<'a, Position>,
        ReadStorage<'a, BlocksTile>,
        Entities<'a>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (mut map, position, blockers, entities) = data;
        map.populate_blocked();
        map.clear_content_index();

        for (entity, position) in (&entities, &position).join() {
            let index = map.xy_index(position.x, position.y);

            // If they block, update the blocking list
            blockers.get(entity).map(|_| map.blocked[index] = true);

            map.tile_content[index].push(entity);
        }
    }
}
