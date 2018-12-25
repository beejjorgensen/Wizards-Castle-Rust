pub struct Player {
    pub x: usize,
    pub y: usize,
    pub z: usize,
    //blind: bool,
}

impl Player {
    pub fn new(x: usize, y: usize, z: usize) -> Player {

        Player {
            x,
            y,
            z,
            //blind: false
        }
    }
}