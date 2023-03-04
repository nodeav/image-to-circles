use rand::Rng;
use opencv::{core, imgproc, prelude::*};

#[derive(Debug)]
#[derive(Copy, Clone)]
pub struct Color {
    b: u8,
    g: u8,
    r: u8,
    a: u8,
}

impl Color {
    pub(crate) fn to_opencv(&self) -> core::Scalar {
        // TODO: use U8C4 images instead?
        // TODO: opacity is not really used :(
        // there are workarounds though - see C++ impl
        core::Scalar::new(
            self.b as f64,
            self.g as f64,
            self.r as f64,
            self.a as f64,
        )
    }
}

impl Color {
    fn random() -> Color {
        Color {
            b: rand::thread_rng().gen_range(0..255),
            g: rand::thread_rng().gen_range(0..255),
            r: rand::thread_rng().gen_range(0..255),
            a: rand::thread_rng().gen_range(0..255),
        }
    }
}

#[derive(Debug)]
#[derive(Copy, Clone)]
pub struct Point {
    x: u16,
    y: u16,
}

impl Point {
    fn random(x_max: u16, y_max: u16) -> Point {
        Point {
            x: rand::thread_rng().gen_range(0..x_max),
            y: rand::thread_rng().gen_range(0..y_max),
        }
    }

    fn to_opencv(&self) -> opencv::core::Point {
        core::Point {
            x: self.x as i32,
            y: self.y as i32,
        }
    }
}

#[derive(Debug)]
#[derive(Copy, Clone)]
pub struct Circle {
    pub center: Point,
    pub radius: u16,
    pub color: Color,
}

impl Circle {
    pub fn random(x_max: u16, y_max: u16) -> Circle {
        use std::cmp::max;

        // radius
        let radius = rand::thread_rng().gen_range(0..max(x_max, y_max)) / 16;

        Self {
            radius,
            color: Color::random(),
            center: Point::random(x_max, y_max),
        }
    }

    pub fn draw(&self, mat: &mut Mat) {
        imgproc::circle(
            mat                                           /* image */,
            self.center.to_opencv(),                    /* center */
            self.radius as i32,                         /* radius */
            self.color.to_opencv(),                      /* color */
            -1,                                      /* thickness */
            opencv::imgproc::LineTypes::FILLED as i32/* line type */,
            0,                                           /* shift */
        ).unwrap();
    }
}