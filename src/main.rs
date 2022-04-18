extern crate sdl2;

use rand::Rng;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use std::process;

struct LifeCell {
    rect: Rect,
    color: Color,
    alive: bool,
}

static WIDTH: u32 = 640;
static HEIGHT: u32 = 480;

fn main() {
    let ctx = sdl2::init().unwrap();
    let video_ctx = ctx.video().unwrap();

    let window = match video_ctx
        .window("Conway's Game of Life in Rust", WIDTH, HEIGHT)
        .position_centered()
        .opengl()
        .build(){
            
        Ok(window) => window,
        Err(err) => panic!("Failed to create window: {}", err),
    };

    let mut canvas = match window.into_canvas().present_vsync().build() {
        Ok(canvas) => canvas,
        Err(err) => panic!("Failed to create canvas: {}", err),
    };

    let mut life_cells = create_cells((WIDTH/10) as usize, (HEIGHT/10) as usize);

    let black = sdl2::pixels::Color::RGB(0, 0, 0);

    let mut events = ctx.event_pump().unwrap();

    loop {
        for event in events.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => {
                    process::exit(1);
                }
                _ => {}
            }
        }

        canvas.set_draw_color(black);
        canvas.clear();

        for life_cell in life_cells.iter_mut() {
            if life_cell.alive {

                if at_border(life_cell) {
                    match check_neighbors_at_border(life_cell) {
                        1 => println!("At here"),
                        _ => println!("wtf")
                    }
                } else {
                    check_neighbors(life_cell)
                }

            } else {

            }

            canvas.set_draw_color(life_cell.color);
            canvas.fill_rect(life_cell.rect).unwrap();
        }

        canvas.present();
    }
}

fn create_cells(h_cells: usize, v_cells: usize) -> Vec<LifeCell> {
    let amount = h_cells * v_cells;
    let mut rects = Vec::with_capacity(amount);
    let mut rng = rand::thread_rng();
    let mut alive: bool;

    for i in 0..h_cells as i32 {
        for j in 0..v_cells as i32 {
            rects.push(LifeCell {
                rect: Rect::new(i * h_cells as i32, j * v_cells as i32, 10, 10),

                color: if rng.gen_range(0..10) > 2 {
                    alive = false;
                    sdl2::pixels::Color::RGB(0, 0, 0)
                } else {
                    alive = true;
                    sdl2::pixels::Color::RGB(255, 255, 255)
                },
                alive,
            });
        }
    }

    rects
}


fn at_border(life_cell: &LifeCell) -> bool {
    life_cell.rect.x == 0 || life_cell.rect.y == 0
}

fn check_neighbors_at_border(life_cell: &mut LifeCell) -> u32 {
    if life_cell.rect.x == 0 && life_cell.rect.y == 0 {

    } else if life_cell.rect.x == 0 {

    } else if life_cell.rect.y == 0 {
        
    }
    10
}

fn check_neighbors(life_cell: &mut LifeCell) -> u32 {
    10
}
