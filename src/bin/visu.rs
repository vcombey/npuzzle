extern crate sdl2;
extern crate image;
#[macro_use]
extern crate itertools;

use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::surface::Surface;
use sdl2::pixels::PixelFormatEnum;
use std::time::Duration;
use image::GenericImage;
use image::{SubImage, FilterType};
use sdl2::image::LoadSurface;
use sdl2::rect::Rect;


const WINDOW_SIZE: u32 = 1080;
pub fn main() {
	let mut subs: Vec<Vec<u8>> = Vec::new();
//	let mut images = Vec::new();
	let n = 3;

// 	match image::open("./resources/vcombey.jpg") {
//         Err(e) => println!("{}",e),
//         Ok(mut img) => {
//             let (w, h) = img.dimensions();
//             let mut img = img.resize_exact(WINDOW_SIZE, WINDOW_SIZE, FilterType::Nearest).to_rgb();
// 			let (w, h) = img.dimensions();
//             let SUB_IMG_SIZE = w / n;
// 			eprintln!("{}:{}", w, h);
//             for (i, j) in iproduct!(0..n, 0..n) {
// 				println!("Trying to product sub image at start position: {}:{}", i, j);
//                 let sub = SubImage::new(&mut img, j * SUB_IMG_SIZE, i * SUB_IMG_SIZE, SUB_IMG_SIZE, SUB_IMG_SIZE)
// 					.to_image().to_vec();
// //                images.push((sub, );
//             }
//         }
//     }

	// let SUB_IMG_SIZE = WINDOW_SIZE / n;
	// for sub in subs.into_iter() {
	// 	images.push((&mut sub, Surface::from_data(&mut sub, SUB_IMG_SIZE, SUB_IMG_SIZE, 1, PixelFormatEnum::RGB555).unwrap()));
	// }
	let image = Surface::from_file("./resources/aalves.jpg").unwrap();
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

	const WINDOW_WIDTH: u32 = 1080;
	const WINDOW_HEIGHT: u32 = 1080;
	
    let mut window = video_subsystem.window("rust-sdl2 demo", WINDOW_WIDTH, WINDOW_HEIGHT)
        .position_centered()
        .build()
        .unwrap();
 
//    let mut canvas = window.into_canvas().build().unwrap();

	
    // canvas.set_draw_color(Color::RGB(0, 255, 255));
    // canvas.clear();
//    canvas.present();
    let mut event_pump = sdl_context.event_pump().unwrap();
    let mut i = 0;
	let mut j = 0;
	let (w, h) = image.size();
	let (sub_w, sub_h) = (WINDOW_WIDTH / 3, WINDOW_HEIGHT / 3);
    'running: loop {
		let rect_src = Rect::new(3740, 1500, sub_w, sub_h);
		let rect_dst = Rect::new(i * sub_w as i32, j * sub_w as i32, sub_w, sub_h);

        // canvas.set_draw_color(Color::RGB(i, 64, 255 - i));
        // canvas.clear();
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
			window_surface.fill_rect(whole_rect, sdl2::pixels::Color::RGB(0, 0, 0));
			image.blit(rect_src, &mut window_surface, rect_dst).unwrap();

			// The rest of the game loop goes here...
			window_surface.update_window();
		}
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
}
