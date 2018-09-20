pub use sdl2::surface::Surface;
pub use sdl2::pixels::Color;
pub use sdl2::rect::Rect;
pub use sdl2::video::WindowSurfaceRef;


pub const WINDOW_SIZE: u32 = 1080;
pub	const WINDOW_WIDTH: u32 = 1080;
pub	const WINDOW_HEIGHT: u32 = 1080;
	


pub trait Visualizable {
	fn visualize(&self, surface: &mut WindowSurfaceRef) -> Result<(), String>;
}
