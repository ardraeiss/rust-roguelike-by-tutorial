use rltk::{Rltk, RGB};
use super::{Rect};

#[derive(PartialEq, Copy, Clone)]
pub enum TileType {
    Wall, Floor
}

pub fn xy_idx(x: i32, y: i32) -> usize {
    (y as usize * 80) + x as usize
}

/// Makes a map with solid boundaries and 400 randomly placed walls. No guarantees that it won't
/// look awful.
#[allow(dead_code)]
pub fn new_map_test() -> Vec<TileType> {
    let mut map = vec![TileType::Floor; 80*50];

    create_boundaries_walls(&mut map);

    // Now we'll randomly splat a bunch of walls. It won't be pretty, but it's a decent illustration.
    // First, obtain the thread-local RNG:
    let mut rng = rltk::RandomNumberGenerator::new();

    for _i in 0..400 {
        let x = rng.roll_dice(1, 79);
        let y = rng.roll_dice(1, 49);
        let idx = xy_idx(x, y);
        if idx != xy_idx(40, 25) {
            map[idx] = TileType::Wall;
        }
    }

    map
}

pub fn new_map_rooms_and_corridors() -> Vec<TileType> {
    let mut map = vec![TileType::Wall; 80*50];

    let room1 = Rect::new(20, 15, 10, 15);
    let room2 = Rect::new(35, 15, 10, 15);

    apply_room_to_map(&room1, &mut map);
    apply_room_to_map(&room2, &mut map);

    map
}

/// Make the boundaries walls
fn create_boundaries_walls(map: &mut [TileType]) {
    for x in 0..80 {
        map[xy_idx(x, 0)] = TileType::Wall;
        map[xy_idx(x, 49)] = TileType::Wall;
    }
    for y in 0..50 {
        map[xy_idx(0, y)] = TileType::Wall;
        map[xy_idx(79, y)] = TileType::Wall;
    }
}

fn apply_room_to_map(room : &Rect, map: &mut [TileType]) {
    for y in room.y1 +1 ..= room.y2 {
        for x in room.x1 + 1 ..= room.x2 {
            map[xy_idx(x, y)] = TileType::Floor;
        }
    }
}

pub fn draw_map(map: &[TileType], ctx : &mut Rltk) {
    let color_floor = (
        RGB::from_f32(0.3, 0.3, 0.35),
        RGB::from_f32(0., 0., 0.),
        rltk::to_cp437('.')
    );
    let color_wall = (
        RGB::from_f32(0.5, 0.6, 0.5),
        RGB::from_f32(0., 0., 0.),
        rltk::to_cp437('#')
    );

    let mut y = 0;
    let mut x = 0;
    for tile in map.iter() {
        // Render a tile depending upon the tile type
        match tile {
            TileType::Floor => {
                ctx.set(x, y, color_floor.0, color_floor.1, color_floor.2);
            }
            TileType::Wall => {
                ctx.set(x, y, color_wall.0, color_wall.1, color_wall.2);
            }
        }

        // Move the coordinates
        x += 1;
        if x > 79 {
            x = 0;
            y += 1;
        }
    }
}
