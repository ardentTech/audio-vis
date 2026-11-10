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

trait FullHeight {
    fn set_rgb(col: usize, row: u32) -> Rgb888;
}

impl <T: FullHeight> ColorPattern for T {
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

pub(crate) struct BlueGreen;
impl FullHeight for BlueGreen {
    fn set_rgb(col: usize, row: u32) -> Rgb888 {
        match row {
            0 | 1 | 2 | 3 | 4 | 5 | 6  | 7 | 8 | 9 | 10 | 11 | 12 => BLUE,
            13 | 14 | 15 | 16 | 17 | 18 | 19 | 20 => OCEAN_BLUE,
            21 | 22 | 23 | 24 | 25 => OCEAN_CYAN,
            26 | 27 | 28 => CYAN,
            29 | 30 => TURQUOISE_CYAN,
            _ => TURQUOISE_GREEN
        }
    }
}

pub(crate) struct RedYellow;

impl FullHeight for RedYellow {
    fn set_rgb(col: usize, row: u32) -> Rgb888 {
        match row {
            0 | 1 | 2 | 3 | 4 | 5 | 6  | 7 | 8 | 9 | 10 | 11 | 12 => RED,
            13 | 14 | 15 | 16 | 17 | 18 | 19 | 20 => RED_ORANGE,
            21 | 22 | 23 | 24 | 25 => ORANGE_RED,
            26 | 27 | 28 => ORANGE,
            29 | 30 => ORANGE_YELLOW,
            _ => WHITE,
        }
    }
}