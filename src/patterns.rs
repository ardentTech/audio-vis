use embedded_graphics::framebuffer::Framebuffer;
use embedded_graphics::geometry::Point;
use embedded_graphics::pixelcolor::Rgb888;
use microfft::Complex32;
use crate::ADC_RESOLUTION;
use crate::colors::*;
use crate::display::{Fb, PANEL_HEIGHT, PANEL_WIDTH};

fn map_range(a: i32, in_min: i32, in_max: i32, out_min: i32, out_max: i32) -> i32 {
    if a < in_min {
        return out_min;
    } else if a > in_max {
        return out_max;
    }
    (a - in_min) * (out_max - out_min) / (in_max - in_min) + out_min
}

pub(crate) enum ColorPattern {
    Cyan,
    Fire,
    Green,
    // CoolMirrored,
    //MardiGras,
    Red,
    // WarmMirrored,
}

impl ColorPattern {
    pub(crate) fn draw(&mut self, fb: &mut Fb, samples: [Complex32; PANEL_WIDTH]) {
        match self {
            ColorPattern::Cyan => draw_mirror_column(fb, samples, CYAN),
            ColorPattern::Fire => draw_mirror_column(fb, samples, FIRE),
            ColorPattern::Green => draw_mirror_column(fb, samples, GREEN_YELLOW),
            // ColorPattern::CoolMirrored => {}
            //ColorPattern::MardiGras => draw_mirror_column(fb, samples, MARDI_GRAS),
            ColorPattern::Red => draw_mirror_column(fb, samples, WARM),
            // ColorPattern::WarmMirrored => {}
        }
    }
}

fn draw_mirror_column(fb: &mut Fb, samples: [Complex32; PANEL_WIDTH], pattern: [Rgb888; 16]) {
    let mut amplitudes: [u32; PANEL_WIDTH] = [0; PANEL_WIDTH];
    samples.iter().enumerate().for_each(|(i, v)| {
        amplitudes[i] = map_range(v.l1_norm() as i32, 0, ADC_RESOLUTION, 0, (PANEL_HEIGHT as i32) / 2) as u32; // TODO so many type casts
    });
    fb.erase();
    for (col, v) in amplitudes.iter().enumerate() {
        for row in 0..*v {
            fb.set_pixel(Point::new(col as i32, ((PANEL_HEIGHT as i32) / 2) - (*v - row) as i32), pattern[row as usize]);
            fb.set_pixel(Point::new(col as i32, ((PANEL_HEIGHT as i32) / 2) + (*v - row - 1) as i32), pattern[row as usize]);
        }
    }
}


static CYAN: [Rgb888; 16] = [
    CYAN_0,
    CYAN_0,
    CYAN_0,
    CYAN_3,
    CYAN_3,
    CYAN_3,
    CYAN_6,
    CYAN_6,
    CYAN_6,
    CYAN_9,
    CYAN_9,
    CYAN_9,
    CYAN_12,
    CYAN_12,
    CYAN_12,
    CYAN_12,
];

static GREEN: [Rgb888; 16] = [
    GREEN_0,
    GREEN_1,
    GREEN_2,
    GREEN_3,
    GREEN_4,
    GREEN_5,
    GREEN_6,
    GREEN_7,
    GREEN_8,
    GREEN_9,
    GREEN_10,
    GREEN_11,
    GREEN_12,
    GREEN_13,
    GREEN_14,
    GREEN_15,
];

static GREEN_YELLOW: [Rgb888; 16] = [
    GREEN_0,
    GREEN_1,
    GREEN_2,
    GREEN_3,
    GREEN_4,
    GREEN_5,
    YELLOW_6,
    YELLOW_7,
    YELLOW_8,
    YELLOW_9,
    YELLOW_10,
    WHITE_11,
    WHITE_12,
    WHITE_13,
    WHITE_14,
    WHITE_15,
];

static WARM: [Rgb888; 16] = [
    RED_0,
    RED_1,
    RED_2,
    RED_3,
    RED_4,
    RED_5,
    RED_6,
    RED_7,
    RED_8,
    RED_9,
    RED_10,
    RED_11,
    RED_12,
    RED_13,
    RED_14,
    RED_15,
];

static FIRE: [Rgb888; 16] = [
    RED_0,
    RED_1,
    RED_2,
    RED_ORANGE_3,
    RED_ORANGE_4,
    RED_ORANGE_5,
    ORANGE_6,
    ORANGE_7,
    ORANGE_8,
    ORANGE_9,
    YELLOW_10,
    YELLOW_11,
    YELLOW_12,
    YELLOW_13,
    WHITE_14,
    WHITE_15,
];