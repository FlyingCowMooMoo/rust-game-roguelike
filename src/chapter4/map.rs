use rltk::{Rltk, RGB};
pub static MAP_SIZE_X: i32 = 80;
pub static MAP_SIZE_Y: i32 = 50;

#[derive(PartialEq, Copy, Clone)]
pub enum TileType {
    Wall,
    Floor,
}


pub fn xy_index(x: i32, y: i32) -> usize {
    (y as usize * MAP_SIZE_X as usize) + x as usize
}


pub fn new_map() -> Vec<TileType> {
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