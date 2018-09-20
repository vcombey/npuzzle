pub use sdl2::surface::Surface;
pub use sdl2::pixels::Color;
pub use sdl2::rect::Rect;
pub use sdl2::video::WindowSurfaceRef;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::{PixelFormatEnum};
use std::time::Duration;
use sdl2::image::LoadSurface;
use sdl2;


use taquin::{Taquin, Dir};

pub const WINDOW_SIZE: u32 = 1080;
pub	const WINDOW_WIDTH: u32 = 1080;
pub	const WINDOW_HEIGHT: u32 = 1080;
use std::path::Path;
	
pub trait Visualizable {
	fn visualize(&self, surface: &mut WindowSurfaceRef, image_ref: Option<&Surface>, goal_taquin: &Taquin) -> Result<(), String>;
}

pub fn visualize_path<P: AsRef<Path>>(path: Vec<Taquin>, image_path: P) -> Result<(), ()> {
	let image = Surface::from_file(image_path).unwrap();
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
	let mut taquin = Taquin::new(3, (0..9).collect::<Vec<u64>>());
	let spiral = Taquin::spiral(3);
	let mut solve_states = path.iter();
	let mut finished = false;
	let mut frame_duration = Duration::new(0, 1_000_000_000u32 / 60);
	let duration_granularity = Duration::new(0, 1_000_000_000u32 / 20);
	let mut playing = false;
	let mut playing_taquin = path.iter().nth(0).unwrap().clone();
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
					if let Some(new_state) = playing_taquin.move_piece(Dir::Down) {
						playing_taquin = new_state;
					}
                },
				Event::KeyDown { keycode: Some(Keycode::Up), .. } => {
					if let Some(new_state) = playing_taquin.move_piece(Dir::Up) {
						playing_taquin = new_state;
					}
                },
				Event::KeyDown { keycode: Some(Keycode::Left), .. } => {
					if let Some(new_state) = playing_taquin.move_piece(Dir::Left) {
						playing_taquin = new_state;
					}
                },
				Event::KeyDown { keycode: Some(Keycode::Right), .. } => {
					if let Some(new_state) = playing_taquin.move_piece(Dir::Right) {
						playing_taquin = new_state;
					}
                },
				Event::KeyDown { keycode: Some(Keycode::R), .. } => {
					finished = false;
					solve_states = path.iter();
					playing_taquin = path.iter().nth(0).unwrap().clone();
                },
				Event::KeyDown { keycode: Some(Keycode::P), .. } => {
					playing ^= true;
					finished = false;
                },
				Event::KeyDown { keycode: Some(Keycode::Equals), .. } => {
					frame_duration += duration_granularity;
					println!("Duration between states: {};{}", frame_duration.as_secs(), frame_duration.subsec_millis());
                },
				Event::KeyDown { keycode: Some(Keycode::Minus), .. } => {
					if frame_duration >= duration_granularity {
						frame_duration -= duration_granularity;
						println!("Duration between states: {};{}", frame_duration.as_secs(), frame_duration.subsec_millis());
					}
                },
                _ => {}
            }
        }
		{
			let mut window_surface = window.surface(&event_pump).unwrap();
			if playing == false {
				if finished == false {
					let whole_rect = window_surface.rect();
					window_surface.fill_rect(whole_rect, sdl2::pixels::Color::RGB(0, 0, 0)).unwrap();

					if let Some(current_taquin) = solve_states.next(){
						current_taquin.visualize(&mut window_surface, Some(&image), &spiral).unwrap();
					} else {
						let window_rect = window_surface.rect();
						image.blit(image.rect(), &mut window_surface, window_rect);
						finished = true;
					}
				}
			} else {
				playing_taquin.visualize(&mut window_surface, Some(&image), &spiral).unwrap();
				if playing_taquin == spiral {
					println!("Congratulation, you finished this puzzle");
				}
			}
			window_surface.update_window().unwrap();
		}
        ::std::thread::sleep(frame_duration);
    }
	Ok(())
}
