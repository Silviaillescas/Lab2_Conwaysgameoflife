pub struct Framebuffer {
    pub buffer: Vec<u32>,
    width: usize,
    height: usize,
    cell_size: usize,
}

impl Framebuffer {
    pub fn new(width: usize, height: usize, cell_size: usize) -> Self {
        Self {
            buffer: vec![0; width * height * cell_size * cell_size],
            width,
            height,
            cell_size,
        }
    }

    pub fn draw_world(&mut self, world: &Vec<Vec<u8>>) {
        for y in 0..self.height {
            for x in 0..self.width {
                let color = if world[y][x] == 1 { crate::color::get_color(x, y) } else { 0x000000 }; // Cambiar colores
                for dy in 0..self.cell_size {
                    for dx in 0..self.cell_size {
                        self.buffer[((y * self.cell_size + dy) * self.width * self.cell_size) + (x * self.cell_size + dx)] = color;
                    }
                }
            }
        }
    }
}
