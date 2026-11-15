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
static THIRD_EYE_I: AtomicU8 = AtomicU8::new(0);

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
    FireIce,
    Green,
    Red,
    Swamp,
    ThirdEye,
}

impl ColorPattern {
    pub(crate) fn draw(&mut self, fb: &mut Fb, samples: [Complex32; PANEL_WIDTH]) {
        match self {
            ColorPattern::Cyan => draw_mirror_column(fb, samples, CYAN_FADE_IN),
            ColorPattern::Fire => {
                match FIRE_I.load(Ordering::Relaxed) {
                    0 => {
                        draw_split_column(fb, samples, RED_ORANGE_FADE_IN, YELLOW_FADE_IN);
                        FIRE_I.store(1, Ordering::Relaxed);
                    },
                    _ => {
                        draw_split_column(fb, samples, RED_FADE_IN, ORANGE_FADE_IN);
                        FIRE_I.store(0, Ordering::Relaxed);
                    },
                }
            },
            ColorPattern::FireIce => {
                draw_split_column(fb, samples, RED_ORANGE_FADE_IN, OCEAN_FADE_IN);
            },
            ColorPattern::Green => draw_mirror_column(fb, samples, GREEN_YELLOW),
            ColorPattern::Red => draw_mirror_column(fb, samples, RED_FADE_IN),
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
            ColorPattern::ThirdEye => {
                match THIRD_EYE_I.load(Ordering::Relaxed) {
                    0 => {
                        draw_mirror_column(fb, samples, BLUE_VIOLET_FADE_IN);
                        THIRD_EYE_I.store(1, Ordering::Relaxed);
                    },
                    1 => {
                        draw_mirror_column(fb, samples, BLUE_FADE_IN);
                        THIRD_EYE_I.store(2, Ordering::Relaxed);
                    },
                    _ => {
                        draw_mirror_column(fb, samples, OCEAN_FADE_IN);
                        THIRD_EYE_I.store(0, Ordering::Relaxed);
                    }
                }
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
    OCEAN_0,
    OCEAN_1,
    OCEAN_2,
    OCEAN_3,
    OCEAN_4,
    OCEAN_5,
    OCEAN_6,
    OCEAN_7,
    OCEAN_8,
    OCEAN_9,
    OCEAN_10,
    OCEAN_11,
    OCEAN_12,
    OCEAN_13,
    OCEAN_14,
    OCEAN_15,
];

static CYAN_FADE_IN: [Rgb888; 16] = [
    CYAN_0,
    CYAN_1,
    CYAN_2,
    CYAN_3,
    CYAN_4,
    CYAN_5,
    CYAN_6,
    CYAN_7,
    CYAN_8,
    CYAN_9,
    CYAN_10,
    CYAN_11,
    CYAN_12,
    CYAN_13,
    CYAN_14,
    CYAN_15,
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

static RED_ORANGE_FADE_IN: [Rgb888; 16] = [
    RED_ORANGE_0,
    RED_ORANGE_1,
    RED_ORANGE_2,
    RED_ORANGE_3,
    RED_ORANGE_4,
    RED_ORANGE_5,
    RED_ORANGE_6,
    RED_ORANGE_7,
    RED_ORANGE_8,
    RED_ORANGE_9,
    RED_ORANGE_10,
    RED_ORANGE_11,
    RED_ORANGE_12,
    RED_ORANGE_13,
    RED_ORANGE_14,
    RED_ORANGE_15,
];

// static FIRE: [Rgb888; 16] = [
//     RED_0,
//     RED_1,
//     RED_2,
//     RED_ORANGE_3,
//     RED_ORANGE_4,
//     RED_ORANGE_5,
//     ORANGE_6,
//     ORANGE_7,
//     ORANGE_8,
//     ORANGE_9,
//     YELLOW_10,
//     YELLOW_11,
//     YELLOW_12,
//     YELLOW_13,
//     WHITE_14,
//     WHITE_15,
// ];

static ORANGE_FADE_IN: [Rgb888; 16] = [
    ORANGE_0,
    ORANGE_1,
    ORANGE_2,
    ORANGE_3,
    ORANGE_4,
    ORANGE_5,
    ORANGE_6,
    ORANGE_7,
    ORANGE_8,
    ORANGE_9,
    ORANGE_10,
    ORANGE_11,
    ORANGE_12,
    ORANGE_13,
    ORANGE_14,
    ORANGE_15,
];

static YELLOW_FADE_IN: [Rgb888; 16] = [
    YELLOW_0,
    YELLOW_1,
    YELLOW_2,
    YELLOW_3,
    YELLOW_4,
    YELLOW_5,
    YELLOW_6,
    YELLOW_7,
    YELLOW_8,
    YELLOW_9,
    YELLOW_10,
    YELLOW_11,
    YELLOW_12,
    YELLOW_13,
    YELLOW_14,
    YELLOW_15,
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

static BLUE_VIOLET_FADE_IN: [Rgb888; 16] = [
    BLUE_VIOLET_1,
    BLUE_VIOLET_2,
    BLUE_VIOLET_3,
    BLUE_VIOLET_4,
    BLUE_VIOLET_5,
    BLUE_VIOLET_6,
    BLUE_VIOLET_7,
    BLUE_VIOLET_8,
    BLUE_VIOLET_9,
    BLUE_VIOLET_10,
    BLUE_VIOLET_11,
    BLUE_VIOLET_12,
    BLUE_VIOLET_13,
    BLUE_VIOLET_14,
    BLUE_VIOLET_15,
    BLUE_VIOLET_15,
];

static BLUE_FADE_IN: [Rgb888; 16] = [
    BLUE_0,
    BLUE_1,
    BLUE_2,
    BLUE_3,
    BLUE_4,
    BLUE_5,
    BLUE_6,
    BLUE_7,
    BLUE_8,
    BLUE_9,
    BLUE_10,
    BLUE_11,
    BLUE_12,
    BLUE_13,
    BLUE_14,
    BLUE_15,
];

static WHITE_FADE_IN: [Rgb888; 16] = [
    WHITE_0,
    WHITE_1,
    WHITE_2,
    WHITE_3,
    WHITE_4,
    WHITE_5,
    WHITE_6,
    WHITE_7,
    WHITE_8,
    WHITE_9,
    WHITE_10,
    WHITE_11,
    WHITE_12,
    WHITE_13,
    WHITE_14,
    WHITE_15,
];