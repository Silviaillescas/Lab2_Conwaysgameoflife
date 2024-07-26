mod framebuffer;
mod color;

use minifb::{Key, Window, WindowOptions};
use std::time::Duration;

const WIDTH: usize = 70;
const HEIGHT: usize = 70;
const CELL_SIZE: usize = 5;  // Tamaño de las células a 5

fn main() {
    let mut window = Window::new(
        "Conway's Game of Life",
        WIDTH * CELL_SIZE,
        HEIGHT * CELL_SIZE,
        WindowOptions {
            scale: minifb::Scale::X1,
            ..WindowOptions::default()
        },
    )
    .unwrap_or_else(|e| {
        panic!("Unable to open Window: {}", e);
    });

    let mut framebuffer = framebuffer::Framebuffer::new(WIDTH, HEIGHT, CELL_SIZE);
    let mut world = vec![vec![0; WIDTH]; HEIGHT];

    // Añadir varios patrones iniciales en diferentes posiciones
    let patterns = vec![
        create_block(1, 1),
        create_beehive(5, 1),
        create_blinker(10, 1),
        create_glider(15, 1),
        create_lwss(20, 1),
        create_toad(1, 10),
        create_pulsar(10, 10),
        create_glider(20, 10),
        create_blinker(1, 20),
        create_toad(5, 20),
        create_block(10, 20),
        create_beehive(15, 20),
        create_glider(20, 20),
        create_lwss(25, 25),
        create_pulsar(5, 25),
        create_toad(10, 25),
        create_blinker(15, 25),
        create_block(20, 25),
        create_beehive(25, 25),
        create_pulsar(30, 30),
        create_glider(35, 35),
        create_lwss(40, 40),
        create_toad(45, 45),
        create_block(50, 50),
        create_beehive(55, 55),
        create_blinker(60, 60),
        create_glider(65, 65),
    ];

    for pattern in patterns {
        for (x, y) in pattern {
            if x < WIDTH && y < HEIGHT {
                world[y][x] = 1;
            }
        }
    }

    while window.is_open() && !window.is_key_down(Key::Escape) {
        update_world(&mut world);
        framebuffer.draw_world(&world);

        window
            .update_with_buffer(&framebuffer.buffer, WIDTH * CELL_SIZE, HEIGHT * CELL_SIZE)
            .unwrap();
        
        std::thread::sleep(Duration::from_millis(50)); // Ajustamos la velocidad de la simulación
    }
}

fn create_block(x: usize, y: usize) -> Vec<(usize, usize)> {
    vec![
        (x, y), (x+1, y),
        (x, y+1), (x+1, y+1),
    ]
}

fn create_beehive(x: usize, y: usize) -> Vec<(usize, usize)> {
    vec![
        (x+1, y), (x+2, y),
        (x, y+1), (x+3, y+1),
        (x+1, y+2), (x+2, y+2),
    ]
}

fn create_blinker(x: usize, y: usize) -> Vec<(usize, usize)> {
    vec![
        (x, y), (x, y+1), (x, y+2)
    ]
}

fn create_toad(x: usize, y: usize) -> Vec<(usize, usize)> {
    vec![
        (x, y), (x+1, y), (x+2, y),
        (x+1, y-1), (x+2, y-1), (x+3, y-1),
    ]
}

fn create_glider(x: usize, y: usize) -> Vec<(usize, usize)> {
    vec![
        (x, y+1), (x+1, y+2),
        (x+2, y), (x+2, y+1), (x+2, y+2),
    ]
}

fn create_lwss(x: usize, y: usize) -> Vec<(usize, usize)> {
    vec![
        (x+1, y), (x+2, y), (x+3, y),
        (x, y+1), (x+3, y+1),
        (x+3, y+2),
        (x, y+3), (x+2, y+3),
    ]
}

fn create_pulsar(x: usize, y: usize) -> Vec<(usize, usize)> {
    let mut cells = vec![];
    for &dx in &[0, 5, 7, 12] {
        for &dy in &[2, 3, 4, 8, 9, 10] {
            if x+dx < WIDTH && y+dy < HEIGHT {
                cells.push((x+dx, y+dy));
                cells.push((x+dy, y+dx));
            }
        }
    }
    for &dx in &[2, 3, 4, 8, 9, 10] {
        for &dy in &[0, 5, 7, 12] {
            if x+dx < WIDTH && y+dy < HEIGHT {
                cells.push((x+dx, y+dy));
                cells.push((x+dy, y+dx));
            }
        }
    }
    cells
}

fn update_world(world: &mut Vec<Vec<u8>>) {
    let mut new_world = world.clone();

    for y in 0..HEIGHT {
        for x in 0..WIDTH {
            let live_neighbors = count_live_neighbors(world, x, y);

            if world[y][x] == 1 {
                if live_neighbors < 2 || live_neighbors > 3 {
                    new_world[y][x] = 0;
                }
            } else {
                if live_neighbors == 3 {
                    new_world[y][x] = 1;
                }
            }
        }
    }

    *world = new_world;
}

fn count_live_neighbors(world: &Vec<Vec<u8>>, x: usize, y: usize) -> u8 {
    let mut count = 0;

    for dy in [-1, 0, 1].iter() {
        for dx in [-1, 0, 1].iter() {
            if *dx == 0 && *dy == 0 {
                continue;
            }

            let nx = (x as isize + dx).rem_euclid(WIDTH as isize) as usize;
            let ny = (y as isize + dy).rem_euclid(HEIGHT as isize) as usize;

            count += world[ny][nx];
        }
    }

    count
}
