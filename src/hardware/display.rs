pub const CELL_SIZE: usize = 15;

pub struct Display
{
    pixels: [[bool; 32]; 64],
}

impl Display
{
    pub fn new() -> Self
    {
        Display {
            pixels: [[false; 32]; 64],
        }
    }

    pub fn get_pixel(&self, x: usize, y: usize) -> bool
    {
        self.pixels[x][y]
    }

    pub fn clear_display(&mut self)
    {
        self.pixels = [[false; 32]; 64];
    }

    pub fn set_pixel(&mut self, x: usize, y: usize, state: bool)
    {
        self.pixels[x][y] = state;
    }
}
