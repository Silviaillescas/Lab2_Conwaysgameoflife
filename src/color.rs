pub fn get_color(x: usize, y: usize) -> u32 {
    let r = ((x + y) * 3 % 255) as u32;
    let g = ((x * y) % 255) as u32;
    let b = (255 - ((x + y) * 5 % 255)) as u32;
    (r << 16) | (g << 8) | b
}
