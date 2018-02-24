//! Basic hello world example.

extern crate ggez;
extern crate image;

use image::*;

use std::fs::File;
use std::env;
use std::path;
use std::cmp;
use ggez::*;
use ggez::event::*;
use ggez::graphics::Drawable;
use ggez::graphics::*;
use taquin::Taquin;
use taquin::Dir;

pub struct MainState {
    frames: usize,
    images: Vec<graphics::Image>,
    spiral: Taquin,
    taquin: Taquin,
}

pub const WINDOW_SIZE: u32 = 300;

impl MainState {
    pub fn new(ctx: &mut Context, taquin: Taquin) -> GameResult<MainState> {
        let n = 3;
        let mut images = Vec::new();

        match image::open("./resources/vcombey.jpg") {
            Err(e) => println!("{}",e),
            Ok(mut img) => {
                let (w, h) = img.dimensions();
                let mut img = img.resize_exact(WINDOW_SIZE, WINDOW_SIZE, FilterType::Nearest);
                let (w, h) = img.dimensions();
                let SUB_IMG_SIZE = w / n;
                for (i, j) in iproduct!(0..n,0..n) {
                    let sub = SubImage::new(&mut img, j * SUB_IMG_SIZE, i * SUB_IMG_SIZE, SUB_IMG_SIZE, SUB_IMG_SIZE).to_image();
                    //let sub = SubImage::new(&mut img, 0, 0, 133, 133).to_image();
                    //let ref mut fout = File::create("test.jpg").unwrap();

                    //resized.save(fout, ImageFormat::JPEG).unwrap();
                    sub.save(format!("./resources/test{}{}.jpg", j, i)).unwrap();
                    let image = graphics::Image::new(ctx, format!("/test{}{}.jpg", j, i)).unwrap();
                    images.push(image);
                }
            }
        }

        let s = MainState {
            frames: 0,
            images,
            spiral: Taquin::spiral(3),
            taquin,
        };
        Ok(s)
    }
}

impl event::EventHandler for MainState {
    fn update(&mut self, _ctx: &mut Context) -> GameResult<()> {
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        graphics::clear(ctx);
        for (i, nb) in self.taquin.iter().enumerate() {
        //for (i, image) in self.images.iter().enumerate() { 
            if *nb == 0 {
                continue ;
            }
            let index = self.taquin.get_index_spiral(*nb, &self.spiral);
            let n = graphics::get_size(ctx).0 as f32;
            let col = (i % 3) as f32;
            let line = (i / 3) as f32;
            self.images[index].draw_ex(ctx, DrawParam{
                dest: Point2::new(n / 3.0 * col as f32, n / 3.0 * line),
                ..Default::default()});
        }

        graphics::present(ctx);
        self.frames += 1;
        if (self.frames % 100) == 0 {
            println!("FPS: {}", ggez::timer::get_fps(ctx));
        }
        Ok(())
    }

        fn key_up_event(&mut self, _ctx: &mut Context, keycode: Keycode, keymod: Mod, repeat: bool) {
            println!("Key released: {:?}, modifier {:?}, repeat: {}",
                     keycode,
                     keymod,
                     repeat);
            if let Some(dir) =  Dir::from_keycode(keycode) {
                if let Some(taquin_move) = self.taquin.move_piece(dir) {
                    self.taquin = taquin_move;
                }
            }
        }

    }
