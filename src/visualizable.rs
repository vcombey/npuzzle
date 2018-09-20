pub use sdl2::surface::Surface;
pub use sdl2::pixels::Color;
pub use sdl2::rect::Rect;
pub use sdl2::video::WindowSurfaceRef;
use taquin::Taquin;

pub const WINDOW_SIZE: u32 = 1080;
pub	const WINDOW_WIDTH: u32 = 1080;
pub	const WINDOW_HEIGHT: u32 = 1080;
use std::path::Path;
	
pub trait Visualizable {
	fn visualize(&self, surface: &mut WindowSurfaceRef, image_ref: Option<&Surface>, goal_taquin: &Taquin) -> Result<(), String>;
}

pub fn visualize_path<P: AsRef<Path>>(path: Vec<Taquin>, image_path: P) {

}
