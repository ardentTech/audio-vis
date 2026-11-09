use embedded_graphics::geometry::Point;
use embedded_graphics::pixelcolor::Rgb888;
use microfft::Complex32;
use crate::{warm_column_mirror, ADC_RESOLUTION};
use crate::colors::{BLUE, BLUE_OCEAN, BLUE_VIOLET, CYAN, CYAN_OCEAN, CYAN_TURQUOISE, GREEN, GREEN_SPRING_GREEN, GREEN_TURQUOISE, OCEAN, OCEAN_BLUE, OCEAN_CYAN, ORANGE, ORANGE_RED, ORANGE_YELLOW, RED, RED_ORANGE, ROSE_RED, SPRING_GREEN_GREEN, TURQUOISE, TURQUOISE_CYAN, TURQUOISE_GREEN, WHITE, YELLOW, YELLOW_ORANGE};
use crate::display::{Fb, PANEL_HEIGHT, PANEL_WIDTH};

fn map_range(a: i32, in_min: i32, in_max: i32, out_min: i32, out_max: i32) -> i32 {
    if a < in_min {
        return out_min;
    } else if a > in_max {
        return out_max;
    }
    (a - in_min) * (out_max - out_min) / (in_max - in_min) + out_min
}

pub(crate) trait ColorPattern {
    fn draw(fb: &mut Fb, samples: [Complex32; PANEL_WIDTH]);
}

pub(crate) struct BlueGreen;

impl ColorPattern for BlueGreen {
    fn draw(fb: &mut Fb, samples: [Complex32; PANEL_WIDTH]) {
        // map range
        let mut amplitudes: [u32; PANEL_WIDTH] = [0; PANEL_WIDTH];
        samples.iter().enumerate().for_each(|(i, v)| {
            amplitudes[i] = map_range(v.l1_norm() as i32, 0, ADC_RESOLUTION, 0, PANEL_HEIGHT as i32) as u32; // TODO so many type casts
        });
        fb.erase();
        for (col, v) in amplitudes.iter().enumerate() {
            for row in 0..*v {
                fb.set_pixel(Point::new(col as i32, (PANEL_HEIGHT as i32 - 1) - row as i32), Self::set_rgb(col, row));
            }
        }
    }
}

impl BlueGreen {
    fn set_rgb(col: usize, row: u32) -> Rgb888 {
        match row {
            0 | 1 => BLUE_VIOLET,
            2 | 3 => BLUE,
            4 | 5 => BLUE_OCEAN,
            6 | 7 => OCEAN_BLUE,
            8 | 9 => OCEAN,
            10 | 11 => OCEAN_CYAN,
            12 | 13 => CYAN_OCEAN,
            14 | 15 => CYAN,
            16 | 17 => CYAN_TURQUOISE,
            18 | 19 => TURQUOISE_CYAN,
            20 | 21 => TURQUOISE,
            22 | 23 => TURQUOISE_GREEN,
            24 | 25 => GREEN_TURQUOISE,
            26 | 27 => GREEN,
            28 | 29 => GREEN_SPRING_GREEN,
            _ => SPRING_GREEN_GREEN
        }
    }
}

pub(crate) struct RedYellow;

impl ColorPattern for RedYellow {
    fn draw(fb: &mut Fb, samples: [Complex32; PANEL_WIDTH]) {
        // map range
        let mut amplitudes: [u32; PANEL_WIDTH] = [0; PANEL_WIDTH];
        samples.iter().enumerate().for_each(|(i, v)| {
            amplitudes[i] = map_range(v.l1_norm() as i32, 0, ADC_RESOLUTION, 0, PANEL_HEIGHT as i32) as u32; // TODO so many type casts
        });
        fb.erase();
        for (col, v) in amplitudes.iter().enumerate() {
            for row in 0..*v {
                fb.set_pixel(Point::new(col as i32, (PANEL_HEIGHT as i32 - 1) - row as i32), Self::set_rgb(col, row));
            }
        }
    }
}

impl RedYellow {
    fn set_rgb(col: usize, row: u32) -> Rgb888 {
        match row {
            0 | 1 | 2 | 3 | 4 => RED,
            5 | 6 | 7 | 8 | 9 => RED_ORANGE,
            10 | 11 | 12 | 13 | 14 => ORANGE_RED,
            15 | 16 | 17 | 18 | 19 => ORANGE,
            20 | 21 | 22 | 23 | 24 => ORANGE_YELLOW,
            25 | 26 | 27 => YELLOW_ORANGE,
            28 | 29 | 30 => YELLOW,
            _ => WHITE
        }
    }
}