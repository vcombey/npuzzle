extern crate sdl2;
#[macro_use]
extern crate itertools;
extern crate npuzzle;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::surface::Surface;
use sdl2::pixels::{PixelFormatEnum, Color};
use std::time::Duration;
use sdl2::image::LoadSurface;
use sdl2::rect::Rect;


use npuzzle::{taquin, taquin::Taquin};
use npuzzle::visualizable::*;


pub fn main() {
	let image = Surface::from_file("./resources/aalves.jpg").unwrap();
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let mut window = video_subsystem.window("rust-sdl2 demo", WINDOW_WIDTH, WINDOW_HEIGHT)
        .position_centered()
        .build()
        .unwrap();
 
    let mut event_pump = sdl_context.event_pump().unwrap();
    let mut i = 0;
	let mut j = 0;
	let (w, h) = image.size();
	let (sub_w, sub_h) = (WINDOW_WIDTH / 3, WINDOW_HEIGHT / 3);
	let taquin = Taquin::new(3, (0..9).collect::<Vec<u64>>());
    'running: loop {
		let rect_src = Rect::new(3740, 1500, sub_w, sub_h);
		let rect_dst = Rect::new(i * sub_w as i32, j * sub_w as i32, sub_w, sub_h);

        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} |
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    break 'running
                },
				Event::KeyDown { keycode: Some(Keycode::Down), .. } => {
					if j == 2 {
						break;
					}
					j += 1;
                },
				Event::KeyDown { keycode: Some(Keycode::Up), .. } => {
					if j == 0 {
						break;
					}
					j -= 1;
					j %= 3;
                },
				Event::KeyDown { keycode: Some(Keycode::Left), .. } => {
					if i == 0 {
						break;
					}
					i -= 1;
					i %= 3;
                },
				Event::KeyDown { keycode: Some(Keycode::Right), .. } => {
					if i == 2 {
						break ;
					}
					i += 1;
                 },
                _ => {}
            }
        }
		{
			
			let mut window_surface = window.surface(&event_pump).unwrap();
			let whole_rect = window_surface.rect();
			window_surface.fill_rect(whole_rect, sdl2::pixels::Color::RGB(0, 0, 0)).unwrap();
			taquin.visualize(&mut window_surface).unwrap();
			window_surface.update_window().unwrap();
//			 image.blit(rect_src, &mut window_surface, rect_dst).unwrap();
			::std::thread::sleep(Duration::new(3,0));
		}
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
}
