const HEIGHT: u32 = 1900;
const WIDTH: u32 = 1080;

type Color = (u8, u8, u8);

struct Building {
    position: u32,
    height: u32,
    width: u32,
    z_index: u8,
    color: Color,
}

impl Building {
    fn generate(position: u32) -> Self {
        // TODO: Randomize.
        Self {
            position,
            height: 0,
            width: 0,
            z_index: 0,
            color: (255, 255, 255),
        }
    }
}

fn main() {
    let mut pointer = 0;
    let mut buildings = Vec::new();

    while pointer < WIDTH {
        let building = Building::generate(pointer);
        buildings.push(building);
        pointer += &building.width;
    }

    println!("{}", buildings.len());
}