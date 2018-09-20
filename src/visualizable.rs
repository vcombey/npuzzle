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

pub fn visualize_path<P: AsRef<Path>>(path: Vec<Taquin>, image_path: P, goal_taquin: &Taquin) -> Result<(), ()> {
	if path.len() == 0 {
		eprintln!("There is no states in the solution path");
		return Err(())
	}
	let image = match Surface::from_file(image_path) {
		Ok(img) => img,
		Err(err_string) => {
			eprintln!("{}", err_string);
			return Err(())
		}
	};
    let sdl_context = match sdl2::init() {
		Ok(c) => c,
		Err(_) => {
			eprintln!("Failed to init SDL2");
			return Err(())
		}
	};
    let video_subsystem = match sdl_context.video() {
		Ok(video) => video,
		Err(err_string) => {
			eprintln!("Failed to init the video subsytem: {}", err_string);
			return Err(())
		}
	};

    let window = match video_subsystem.window("rust-sdl2 demo", WINDOW_WIDTH, WINDOW_HEIGHT)
        .position_centered()
        .build() {
			Ok(win) => win,
			Err(_) => {
				eprintln!("Failed to create window");
				return Err(())
			}
		};
	
    let mut event_pump = match sdl_context.event_pump() {
		Ok(e) => e,
		Err(err_string) => {
			eprintln!("Failed to get the SDL event pump for current window: {}", err_string);
			return Err(())
		}
	};
	let (w, h) = image.size();
	let (sub_w, sub_h) = (WINDOW_WIDTH / 3, WINDOW_HEIGHT / 3);
	let spiral = goal_taquin.clone();
	let mut solve_states = path.iter();
	let mut finished = false;
	let mut frame_duration = Duration::new(0, 1_000_000_000u32 / 60);
	let duration_granularity = Duration::new(0, 1_000_000_000u32 / 20);
	let mut playing = false;
	let mut playing_taquin = path.iter().nth(0).unwrap().clone();
    'running: loop {
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
			let mut window_surface = match window.surface(&event_pump) {
				Ok(surface) => surface,
				Err(_) => {
					eprintln!("SDL2 Internal error");
					return Err(())
				}
			};
			if playing == false {
				if finished == false {
					let whole_rect = window_surface.rect();
					match window_surface.fill_rect(whole_rect, sdl2::pixels::Color::RGB(0, 0, 0)) {
						Ok(_) => (),
						Err(err_string) => {
							eprintln!("Internal SDL2 Error: {}", err_string);
							return Err(())
						}
					};
					if let Some(current_taquin) = solve_states.next() {
						if let Err(err_string) = current_taquin.visualize(&mut window_surface, Some(&image), &spiral) {
							eprintln!("Internal SDL2 Error: {}", err_string);
							return Err(())
						}
					} else {
						let window_rect = window_surface.rect();
						match image.blit(image.rect(), &mut window_surface, window_rect) {
							Ok(_) => (),
							Err(err_string) => {
								eprintln!("Internal SDL2 Error: {}", err_string);
								return Err(())
							}
						}
						finished = true;
					}
				}
			} else {
				if let Err(err_string) = playing_taquin.visualize(&mut window_surface, Some(&image), &spiral) {
					eprintln!("Internal SDL2 Error: {}", err_string);
					return Err(())
				}
				if playing_taquin == spiral {
					println!("Congratulation, you finished this puzzle");
					finished = true;
				}
			}
			match window_surface.update_window() {
				Ok(_) => (),
				Err(err_string) => {
					eprintln!("Internal SDL2 Error: {}", err_string);
					return Err(())
				}
			}
		}
        ::std::thread::sleep(frame_duration);
    }
	Ok(())
}
