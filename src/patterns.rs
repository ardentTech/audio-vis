use core::sync::atomic::{AtomicU8, Ordering};
use embedded_graphics::framebuffer::Framebuffer;
use embedded_graphics::geometry::Point;
use embedded_graphics::pixelcolor::Rgb888;
use microfft::Complex32;
use crate::ADC_RESOLUTION;
use crate::colors::*;
use crate::display::{Fb, PANEL_HEIGHT, PANEL_WIDTH};

static FIRE_I: AtomicU8 = AtomicU8::new(0);
static SWAMP_I: AtomicU8 = AtomicU8::new(0);
static TESS: AtomicU8 = AtomicU8::new(0);

fn map_range(a: i32, in_min: i32, in_max: i32, out_min: i32, out_max: i32) -> i32 {
    if a < in_min {
        return out_min;
    } else if a > in_max {
        return out_max;
    }
    (a - in_min) * (out_max - out_min) / (in_max - in_min) + out_min
}

pub(crate) enum ColorPattern {
    Fire,
    FireIce,
    Swamp,
    Tess,
}

impl ColorPattern {
    pub(crate) fn draw(&mut self, fb: &mut Fb, samples: [Complex32; PANEL_WIDTH]) {
        match self {
            ColorPattern::Fire => {
                match FIRE_I.load(Ordering::Relaxed) {
                    0 => {
                        draw_split_column(fb, samples, ORANGE_FADE_IN, YELLOW_FADE_IN);
                        FIRE_I.store(1, Ordering::Relaxed);
                    },
                    _ => {
                        draw_split_column(fb, samples, RED_FADE_IN, ORANGE_FADE_IN);
                        FIRE_I.store(0, Ordering::Relaxed);
                    },
                }
            },
            ColorPattern::FireIce => {
                draw_split_column(fb, samples, ORANGE_FADE_IN, OCEAN_FADE_IN);
            },
            ColorPattern::Swamp => {
                match SWAMP_I.load(Ordering::Relaxed) {
                    0 => {
                        draw_mirror_column(fb, samples, ORANGE_FADE_IN);
                        SWAMP_I.store(1, Ordering::Relaxed);
                    },
                    1 => {
                        draw_mirror_column(fb, samples, YELLOW_FADE_IN);
                        SWAMP_I.store(2, Ordering::Relaxed);
                    },
                    _ => {
                        draw_mirror_column(fb, samples, SPRING_GREEN_FADE_IN);
                        SWAMP_I.store(0, Ordering::Relaxed);
                    }
                }
            },
            ColorPattern::Tess => {
                draw_split_column(fb, samples, VIOLET_FADE_IN, OCEAN_FADE_IN);
            },
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

fn draw_split_column(fb: &mut Fb, samples: [Complex32; PANEL_WIDTH], pattern1: [Rgb888; 16], pattern2: [Rgb888; 16]) {
    let mut amplitudes: [u32; PANEL_WIDTH] = [0; PANEL_WIDTH];
    samples.iter().enumerate().for_each(|(i, v)| {
        amplitudes[i] = map_range(v.l1_norm() as i32, 0, ADC_RESOLUTION, 0, (PANEL_HEIGHT as i32) / 2) as u32; // TODO so many type casts
    });
    fb.erase();
    for (col, v) in amplitudes.iter().enumerate() {
        for row in 0..*v {
            fb.set_pixel(Point::new(col as i32, ((PANEL_HEIGHT as i32) / 2) - (*v - row) as i32), pattern1[row as usize]);
            fb.set_pixel(Point::new(col as i32, ((PANEL_HEIGHT as i32) / 2) + (*v - row - 1) as i32), pattern2[row as usize]);
        }
    }
}

static OCEAN_FADE_IN: [Rgb888; 16] = [
    Rgb888::new(0, 4, 8),
    Rgb888::new(0, 4, 8),
    Rgb888::new(0, 4, 8),
    Rgb888::new(0, 4, 8),
    Rgb888::new(0, 8, 16),
    Rgb888::new(0, 8, 16),
    Rgb888::new(0, 8, 16),
    Rgb888::new(0, 16, 32),
    Rgb888::new(0, 16, 32),
    Rgb888::new(0, 16, 32),
    Rgb888::new(0, 32, 64),
    Rgb888::new(0, 32, 64),
    Rgb888::new(0, 64, 128),
    Rgb888::new(0, 64, 128),
    Rgb888::new(0, 128, 255),
    Rgb888::new(128, 128, 255),
];

static GREEN_FADE_IN: [Rgb888; 16] = [
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

static RED_FADE_IN: [Rgb888; 16] = [
    Rgb888::new(8, 0, 0),
    Rgb888::new(8, 0, 0),
    Rgb888::new(8, 0, 0),
    Rgb888::new(8, 0, 0),
    Rgb888::new(16, 0, 0),
    Rgb888::new(16, 0, 0),
    Rgb888::new(16, 0, 0),
    Rgb888::new(32, 0, 0),
    Rgb888::new(32, 0, 0),
    Rgb888::new(32, 0, 0),
    Rgb888::new(64, 0, 0),
    Rgb888::new(64, 0, 0),
    Rgb888::new(128, 0, 0),
    Rgb888::new(128, 0, 0),
    Rgb888::new(255, 0, 0),
    Rgb888::new(255, 128, 128),
];

static ORANGE_FADE_IN: [Rgb888; 16] = [
    Rgb888::new(8, 4, 0),
    Rgb888::new(8, 4, 0),
    Rgb888::new(8, 4, 0),
    Rgb888::new(8, 4, 0),
    Rgb888::new(16, 8, 0),
    Rgb888::new(16, 8, 0),
    Rgb888::new(16, 8, 0),
    Rgb888::new(32, 16, 0),
    Rgb888::new(32, 16, 0),
    Rgb888::new(32, 16, 0),
    Rgb888::new(64, 32, 0),
    Rgb888::new(64, 32, 0),
    Rgb888::new(128, 64, 0),
    Rgb888::new(128, 64, 0),
    Rgb888::new(255, 128, 0),
    Rgb888::new(255, 128, 128),
];

static YELLOW_FADE_IN: [Rgb888; 16] = [
    Rgb888::new(8, 8, 0),
    Rgb888::new(8, 8, 0),
    Rgb888::new(8, 8, 0),
    Rgb888::new(8, 8, 0),
    Rgb888::new(16, 16, 0),
    Rgb888::new(16, 16, 0),
    Rgb888::new(16, 16, 0),
    Rgb888::new(32, 32, 0),
    Rgb888::new(32, 32, 0),
    Rgb888::new(32, 32, 0),
    Rgb888::new(64, 64, 0),
    Rgb888::new(64, 64, 0),
    Rgb888::new(128, 128, 0),
    Rgb888::new(128, 128, 0),
    Rgb888::new(255, 255, 0),
    Rgb888::new(255, 255, 128),
];

static SPRING_GREEN_FADE_IN: [Rgb888; 16] = [
    SPRING_GREEN_0,
    SPRING_GREEN_1,
    SPRING_GREEN_2,
    SPRING_GREEN_3,
    SPRING_GREEN_4,
    SPRING_GREEN_5,
    SPRING_GREEN_6,
    SPRING_GREEN_7,
    SPRING_GREEN_8,
    SPRING_GREEN_9,
    SPRING_GREEN_10,
    SPRING_GREEN_11,
    SPRING_GREEN_12,
    SPRING_GREEN_13,
    SPRING_GREEN_14,
    SPRING_GREEN_15,
];

static BLUE_FADE_IN: [Rgb888; 16] = [
    Rgb888::new(0, 0, 8),
    Rgb888::new(0, 0, 8),
    Rgb888::new(0, 0, 8),
    Rgb888::new(0, 0, 8),
    Rgb888::new(0, 0, 16),
    Rgb888::new(0, 0, 16),
    Rgb888::new(0, 0, 16),
    Rgb888::new(0, 0, 32),
    Rgb888::new(0, 0, 32),
    Rgb888::new(0, 0, 32),
    Rgb888::new(0, 0, 64),
    Rgb888::new(0, 0, 64),
    Rgb888::new(0, 0, 128),
    Rgb888::new(0, 0, 128),
    Rgb888::new(0, 0, 255),
    Rgb888::new(128, 128, 255),
];

static VIOLET_FADE_IN: [Rgb888; 16] = [
    Rgb888::new(4, 0, 8),
    Rgb888::new(4, 0, 8),
    Rgb888::new(4, 0, 8),
    Rgb888::new(4, 0, 8),
    Rgb888::new(8, 0, 16),
    Rgb888::new(8, 0, 16),
    Rgb888::new(8, 0, 16),
    Rgb888::new(16, 0, 32),
    Rgb888::new(16, 0, 32),
    Rgb888::new(16, 0, 32),
    Rgb888::new(32, 0, 64),
    Rgb888::new(32, 0, 64),
    Rgb888::new(64, 0, 128),
    Rgb888::new(64, 0, 128),
    Rgb888::new(128, 0, 255),
    Rgb888::new(128, 128, 255),
];

static WHITE_FADE_IN: [Rgb888; 16] = [
    Rgb888::new(2, 2, 2),
    Rgb888::new(2, 2, 2),
    Rgb888::new(4, 4, 4),
    Rgb888::new(4, 4, 4),
    Rgb888::new(8, 8, 8),
    Rgb888::new(8, 8, 8),
    Rgb888::new(16, 16, 16),
    Rgb888::new(16, 16, 16),
    Rgb888::new(32, 32, 32),
    Rgb888::new(32, 32, 32),
    Rgb888::new(64, 64, 64),
    Rgb888::new(64, 64, 64),
    Rgb888::new(128, 128, 128),
    Rgb888::new(128,  128, 128),
    Rgb888::new(255, 255, 255),
    Rgb888::new(255, 255, 255),
];