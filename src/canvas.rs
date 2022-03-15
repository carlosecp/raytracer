#![allow(dead_code)]
#![allow(unused_variables)]
use crate::color::{self, Color};

#[derive(Debug)]
pub struct Canvas {
    width: i32,
    height: i32,
    pixels: Vec<Color>,
}

impl Canvas {
    pub fn new(width: i32, height: i32) -> Self {
        let mut pixels: Vec<Color> = Vec::new();

        for col in 0..(width * height) {
            pixels.push(color::DEFAULT_COLOR);
        }

        Canvas {
            width,
            height,
            pixels,
        }
    }

    pub fn write_pixel(&mut self, x: i32, y: i32, color: &Color) {
        let index = (self.height * x + y) as usize;
        match self.pixels.get(index) {
            Some(_) => self.pixels[index] = *color,
            None => println!("There is no pixel at this location"),
        }
    }

    pub fn pixel_at(&self, x: i32, y: i32) {
    }
}

#[cfg(test)]
mod create {
    use super::*;

    #[test]
    fn creating_canvas() {
        let c = Canvas::new(10, 20);

        for pixel in c.pixels {
            assert_eq!(pixel, color::DEFAULT_COLOR);
        }
    }

    #[test]
    fn write_pixel_to_canvas() {
        let mut c = Canvas::new(10, 20);
        let red = Color::new(1., 0., 0.);

        let x = 2;
        let y = 3;
        let index = (c.height * x + y) as usize;

        c.write_pixel(x, y, &red);
        assert_eq!(c.pixels[index], red);

        let x = 69;
        let y = 69;
        let index = (c.height * x + y) as usize;

        assert_eq!(c.pixels[index], red);
    }
}
