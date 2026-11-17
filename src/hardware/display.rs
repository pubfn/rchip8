pub const WIDTH: usize = 64;
pub const HEIGHT: usize = 32;

pub struct Framebuffer
{
    pixels: [[bool; HEIGHT]; WIDTH],
}

impl Framebuffer
{
    pub fn new() -> Self
    {
        Self {
            pixels: [[false; HEIGHT]; WIDTH],
        }
    }
}

pub trait Display
{
    fn get_pixel(&self, x: usize, y: usize) -> bool;

    fn set_pixel(&mut self, x: usize, y: usize, state: bool);

    fn clear_display(&mut self);
}

impl Display for Framebuffer
{
    fn get_pixel(&self, x: usize, y: usize) -> bool
    {
        self.pixels[x][y]
    }

    fn set_pixel(&mut self, x: usize, y: usize, state: bool)
    {
        self.pixels[x][y] = state;
    }

    fn clear_display(&mut self)
    {
        self.pixels = [[false; HEIGHT]; WIDTH];
    }
}
