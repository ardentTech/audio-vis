use core::sync::atomic::{AtomicU8, Ordering};
use embedded_graphics::geometry::Point;
use embedded_graphics::pixelcolor::Rgb888;
use microfft::Complex32;
use crate::ADC_RESOLUTION;
use crate::curves::{SINE_EASE_IN, SINE_EASE_IN_HALF, SINE_EASE_IN_QUARTER};
use crate::display::{Fb, PANEL_HEIGHT, PANEL_WIDTH};

static MURICA_IDX: AtomicU8 = AtomicU8::new(0);
static MARDI_GRAS_IDX: AtomicU8 = AtomicU8::new(0);

fn map_range(a: i32, in_min: i32, in_max: i32, out_min: i32, out_max: i32) -> i32 {
    if a < in_min {
        return out_min;
    } else if a > in_max {
        return out_max;
    }
    (a - in_min) * (out_max - out_min) / (in_max - in_min) + out_min
}

pub(crate) enum ColorPattern {
    Clean,
    Fire,
    FireIce,
    MardiGras,
    Murica,
    Tess,
}

impl ColorPattern {
    pub(crate) fn draw(&mut self, fb: &mut Fb, samples: [Complex32; PANEL_WIDTH]) {
        match self {
            ColorPattern::Clean => {
                draw_split_column(fb, samples, WHITE_FADE_IN, WHITE_FADE_IN);
            },
            ColorPattern::Fire => {
                draw_mirror_column(fb, samples, FIRE);
            },
            //     match FIRE_I.load(Ordering::Relaxed) {
            //         0 => {
            //             draw_split_column(fb, samples, ORANGE_FADE_IN, YELLOW_FADE_IN);
            //             FIRE_I.store(1, Ordering::Relaxed);
            //         },
            //         _ => {
            //             draw_split_column(fb, samples, RED_ORANGE_FADE_IN, ORANGE_FADE_IN);
            //             FIRE_I.store(0, Ordering::Relaxed);
            //         },
            //     }
            // },
            ColorPattern::FireIce => {
                draw_split_column(fb, samples, RED_ORANGE_FADE_IN, OCEAN_FADE_IN);
            },
            ColorPattern::MardiGras => {
                draw_mardi_gras(fb, samples);
            },
            ColorPattern::Murica => {
                draw_murica(fb, samples);
            },
            ColorPattern::Tess => {
                draw_split_column(fb, samples, VIOLET_FADE_IN, OCEAN_FADE_IN);
            },
        }
    }
}

fn calc_amplitudes(samples: [Complex32; PANEL_WIDTH]) -> [u32; PANEL_WIDTH] {
    let mut amplitudes: [u32; PANEL_WIDTH] = [0; PANEL_WIDTH];
    samples.iter().enumerate().for_each(|(i, v)| {
        amplitudes[i] = map_range(v.l1_norm() as i32, 0, ADC_RESOLUTION, 0, (PANEL_HEIGHT as i32) / 2) as u32; // TODO so many type casts
    });
    amplitudes
}

fn draw_mirror_column(fb: &mut Fb, samples: [Complex32; PANEL_WIDTH], pattern: [Rgb888; 16]) {
    let amplitudes = calc_amplitudes(samples);
    fb.erase();
    for (col, v) in amplitudes.iter().enumerate() {
        for row in 0..*v {
            fb.set_pixel(Point::new(col as i32, ((PANEL_HEIGHT as i32) / 2) - (*v - row) as i32), pattern[row as usize]);
            fb.set_pixel(Point::new(col as i32, ((PANEL_HEIGHT as i32) / 2) + (*v - row - 1) as i32), pattern[row as usize]);
        }
    }
}

fn draw_mardi_gras(fb: &mut Fb, samples: [Complex32; PANEL_WIDTH]) {
    let amplitudes = calc_amplitudes(samples);
    fb.erase();
    let mut idx = MARDI_GRAS_IDX.load(Ordering::Relaxed);

    for (col, v) in amplitudes.iter().enumerate() {
        for row in 0..*v {
            fb.set_pixel(Point::new(col as i32, ((PANEL_HEIGHT as i32) / 2) - (*v - row) as i32), MARDI_GRAS[idx as usize][row as usize]);
            fb.set_pixel(Point::new(col as i32, ((PANEL_HEIGHT as i32) / 2) + (*v - row - 1) as i32), MARDI_GRAS[idx as usize][row as usize]);
        }
        idx = if idx == 3 { 0 } else { idx + 1 };
    }

    MARDI_GRAS_IDX.store(if idx == 3 { 0 } else { idx + 1 }, Ordering::Relaxed);
}

fn draw_murica(fb: &mut Fb, samples: [Complex32; PANEL_WIDTH]) {
    let amplitudes = calc_amplitudes(samples);
    fb.erase();
    let mut idx = MURICA_IDX.load(Ordering::Relaxed);

    for (col, v) in amplitudes.iter().enumerate() {
        for row in 0..*v {
            fb.set_pixel(Point::new(col as i32, ((PANEL_HEIGHT as i32) / 2) - (*v - row) as i32), MURICA[idx as usize][row as usize]);
            fb.set_pixel(Point::new(col as i32, ((PANEL_HEIGHT as i32) / 2) + (*v - row - 1) as i32), MURICA[idx as usize][row as usize]);
        }
        idx = if idx == 2 { 0 } else { idx + 1 };
    }

    MURICA_IDX.store(if idx == 2 { 0 } else { idx + 1 }, Ordering::Relaxed);
}

fn draw_split_column(fb: &mut Fb, samples: [Complex32; PANEL_WIDTH], pattern1: [Rgb888; 16], pattern2: [Rgb888; 16]) {
    let amplitudes = calc_amplitudes(samples);
    fb.erase();
    for (col, v) in amplitudes.iter().enumerate() {
        for row in 0..*v {
            fb.set_pixel(Point::new(col as i32, ((PANEL_HEIGHT as i32) / 2) - (*v - row) as i32), pattern1[row as usize]);
            fb.set_pixel(Point::new(col as i32, ((PANEL_HEIGHT as i32) / 2) + (*v - row - 1) as i32), pattern2[row as usize]);
        }
    }
}

static BLUE_FADE_IN: [Rgb888; 16] = [
    Rgb888::new(0, 0, SINE_EASE_IN[0]),
    Rgb888::new(0, 0, SINE_EASE_IN[1]),
    Rgb888::new(0, 0, SINE_EASE_IN[2]),
    Rgb888::new(0, 0, SINE_EASE_IN[3]),
    Rgb888::new(0, 0, SINE_EASE_IN[4]),
    Rgb888::new(0, 0, SINE_EASE_IN[5]),
    Rgb888::new(0, 0, SINE_EASE_IN[6]),
    Rgb888::new(0, 0, SINE_EASE_IN[7]),
    Rgb888::new(0, 0, SINE_EASE_IN[8]),
    Rgb888::new(0, 0, SINE_EASE_IN[9]),
    Rgb888::new(0, 0, SINE_EASE_IN[10]),
    Rgb888::new(0, 0, SINE_EASE_IN[11]),
    Rgb888::new(0, 0, SINE_EASE_IN[12]),
    Rgb888::new(0, 0, SINE_EASE_IN[13]),
    Rgb888::new(0, 0, SINE_EASE_IN[14]),
    Rgb888::new(0, 0, SINE_EASE_IN[15]),
];

static FIRE: [Rgb888; 16] = [
    RED_FADE_IN[0],
    RED_ORANGE_FADE_IN[1],
    ORANGE_FADE_IN[2],
    ORANGE_FADE_IN[3],
    ORANGE_FADE_IN[4],
    ORANGE_FADE_IN[5],
    YELLOW_ORANGE_FADE_IN[6],
    YELLOW_ORANGE_FADE_IN[7],
    YELLOW_ORANGE_FADE_IN[8],
    YELLOW_ORANGE_FADE_IN[9],
    YELLOW_FADE_IN[10],
    YELLOW_FADE_IN[11],
    YELLOW_FADE_IN[12],
    YELLOW_FADE_IN[13],
    WHITE_FADE_IN[14],
    WHITE_FADE_IN[15],
];

static GREEN_FADE_IN: [Rgb888; 16] = [
    Rgb888::new(0, SINE_EASE_IN[0], 0),
    Rgb888::new(0, SINE_EASE_IN[1], 0),
    Rgb888::new(0, SINE_EASE_IN[2], 0),
    Rgb888::new(0, SINE_EASE_IN[3], 0),
    Rgb888::new(0, SINE_EASE_IN[4], 0),
    Rgb888::new(0, SINE_EASE_IN[5], 0),
    Rgb888::new(0, SINE_EASE_IN[6], 0),
    Rgb888::new(0, SINE_EASE_IN[7], 0),
    Rgb888::new(0, SINE_EASE_IN[8], 0),
    Rgb888::new(0, SINE_EASE_IN[9], 0),
    Rgb888::new(0, SINE_EASE_IN[10], 0),
    Rgb888::new(0, SINE_EASE_IN[11], 0),
    Rgb888::new(0, SINE_EASE_IN[12], 0),
    Rgb888::new(0, SINE_EASE_IN[13], 0),
    Rgb888::new(0, SINE_EASE_IN[14], 0),
    Rgb888::new(0, SINE_EASE_IN[15], 0),
];

static MARDI_GRAS: [[Rgb888; 16]; 4] = [GREEN_FADE_IN, WHITE_FADE_IN, VIOLET_FADE_IN, YELLOW_FADE_IN];

static MURICA: [[Rgb888; 16]; 3] = [RED_FADE_IN, WHITE_FADE_IN, BLUE_FADE_IN];

static OCEAN_FADE_IN: [Rgb888; 16] = [
    Rgb888::new(0, SINE_EASE_IN_HALF[0], SINE_EASE_IN[0]),
    Rgb888::new(0, SINE_EASE_IN_HALF[1], SINE_EASE_IN[1]),
    Rgb888::new(0, SINE_EASE_IN_HALF[2], SINE_EASE_IN[2]),
    Rgb888::new(0, SINE_EASE_IN_HALF[3], SINE_EASE_IN[3]),
    Rgb888::new(0, SINE_EASE_IN_HALF[4], SINE_EASE_IN[4]),
    Rgb888::new(0, SINE_EASE_IN_HALF[5], SINE_EASE_IN[5]),
    Rgb888::new(0, SINE_EASE_IN_HALF[6], SINE_EASE_IN[6]),
    Rgb888::new(0, SINE_EASE_IN_HALF[7], SINE_EASE_IN[7]),
    Rgb888::new(0, SINE_EASE_IN_HALF[8], SINE_EASE_IN[8]),
    Rgb888::new(0, SINE_EASE_IN_HALF[9], SINE_EASE_IN[9]),
    Rgb888::new(0, SINE_EASE_IN_HALF[10], SINE_EASE_IN[10]),
    Rgb888::new(0, SINE_EASE_IN_HALF[11], SINE_EASE_IN[11]),
    Rgb888::new(0, SINE_EASE_IN_HALF[12], SINE_EASE_IN[12]),
    Rgb888::new(0, SINE_EASE_IN_HALF[13], SINE_EASE_IN[13]),
    Rgb888::new(0, SINE_EASE_IN_HALF[14], SINE_EASE_IN[14]),
    Rgb888::new(SINE_EASE_IN_HALF[15], SINE_EASE_IN_HALF[15], SINE_EASE_IN[15]),
];

static ORANGE_FADE_IN: [Rgb888; 16] = [
    Rgb888::new(SINE_EASE_IN[0], SINE_EASE_IN_HALF[0], 0),
    Rgb888::new(SINE_EASE_IN[1], SINE_EASE_IN_HALF[1], 0),
    Rgb888::new(SINE_EASE_IN[2], SINE_EASE_IN_HALF[2], 0),
    Rgb888::new(SINE_EASE_IN[3], SINE_EASE_IN_HALF[3], 0),
    Rgb888::new(SINE_EASE_IN[4], SINE_EASE_IN_HALF[4], 0),
    Rgb888::new(SINE_EASE_IN[5], SINE_EASE_IN_HALF[5], 0),
    Rgb888::new(SINE_EASE_IN[6], SINE_EASE_IN_HALF[6], 0),
    Rgb888::new(SINE_EASE_IN[7], SINE_EASE_IN_HALF[7], 0),
    Rgb888::new(SINE_EASE_IN[8], SINE_EASE_IN_HALF[8], 0),
    Rgb888::new(SINE_EASE_IN[9], SINE_EASE_IN_HALF[9], 0),
    Rgb888::new(SINE_EASE_IN[10], SINE_EASE_IN_HALF[10], 0),
    Rgb888::new(SINE_EASE_IN[11], SINE_EASE_IN_HALF[11], 0),
    Rgb888::new(SINE_EASE_IN[12], SINE_EASE_IN_HALF[12], 0),
    Rgb888::new(SINE_EASE_IN[13], SINE_EASE_IN_HALF[13], 0),
    Rgb888::new(SINE_EASE_IN[14], SINE_EASE_IN_HALF[14], 0),
    Rgb888::new(SINE_EASE_IN[15], SINE_EASE_IN_HALF[15], SINE_EASE_IN_HALF[15]),
];

static RED_FADE_IN: [Rgb888; 16] = [
    Rgb888::new(SINE_EASE_IN[0], 0, 0),
    Rgb888::new(SINE_EASE_IN[1], 0, 0),
    Rgb888::new(SINE_EASE_IN[2], 0, 0),
    Rgb888::new(SINE_EASE_IN[3], 0, 0),
    Rgb888::new(SINE_EASE_IN[4], 0, 0),
    Rgb888::new(SINE_EASE_IN[5], 0, 0),
    Rgb888::new(SINE_EASE_IN[6], 0, 0),
    Rgb888::new(SINE_EASE_IN[7], 0, 0),
    Rgb888::new(SINE_EASE_IN[8], 0, 0),
    Rgb888::new(SINE_EASE_IN[9], 0, 0),
    Rgb888::new(SINE_EASE_IN[10], 0, 0),
    Rgb888::new(SINE_EASE_IN[11], 0, 0),
    Rgb888::new(SINE_EASE_IN[12], 0, 0),
    Rgb888::new(SINE_EASE_IN[13], 0, 0),
    Rgb888::new(SINE_EASE_IN[14], 0, 0),
    Rgb888::new(SINE_EASE_IN[15], 0, 0),
];

static RED_ORANGE_FADE_IN: [Rgb888; 16] = [
    Rgb888::new(SINE_EASE_IN[0], SINE_EASE_IN_QUARTER[0], 0),
    Rgb888::new(SINE_EASE_IN[1], SINE_EASE_IN_QUARTER[1], 0),
    Rgb888::new(SINE_EASE_IN[2], SINE_EASE_IN_QUARTER[2], 0),
    Rgb888::new(SINE_EASE_IN[3], SINE_EASE_IN_QUARTER[3], 0),
    Rgb888::new(SINE_EASE_IN[4], SINE_EASE_IN_QUARTER[4], 0),
    Rgb888::new(SINE_EASE_IN[5], SINE_EASE_IN_QUARTER[5], 0),
    Rgb888::new(SINE_EASE_IN[6], SINE_EASE_IN_QUARTER[6], 0),
    Rgb888::new(SINE_EASE_IN[7], SINE_EASE_IN_QUARTER[7], 0),
    Rgb888::new(SINE_EASE_IN[8], SINE_EASE_IN_QUARTER[8], 0),
    Rgb888::new(SINE_EASE_IN[9], SINE_EASE_IN_QUARTER[9], 0),
    Rgb888::new(SINE_EASE_IN[10], SINE_EASE_IN_QUARTER[10], 0),
    Rgb888::new(SINE_EASE_IN[11], SINE_EASE_IN_QUARTER[11], 0),
    Rgb888::new(SINE_EASE_IN[12], SINE_EASE_IN_QUARTER[12], 0),
    Rgb888::new(SINE_EASE_IN[13], SINE_EASE_IN_QUARTER[13], 0),
    Rgb888::new(SINE_EASE_IN[14], SINE_EASE_IN_QUARTER[14], 0),
    Rgb888::new(SINE_EASE_IN[15], SINE_EASE_IN_QUARTER[15], SINE_EASE_IN_QUARTER[15]),
];

static VIOLET_FADE_IN: [Rgb888; 16] = [
    Rgb888::new(SINE_EASE_IN_HALF[0], 0, SINE_EASE_IN[0]),
    Rgb888::new(SINE_EASE_IN_HALF[1], 0, SINE_EASE_IN[1]),
    Rgb888::new(SINE_EASE_IN_HALF[2], 0, SINE_EASE_IN[2]),
    Rgb888::new(SINE_EASE_IN_HALF[3], 0, SINE_EASE_IN[3]),
    Rgb888::new(SINE_EASE_IN_HALF[4], 0, SINE_EASE_IN[4]),
    Rgb888::new(SINE_EASE_IN_HALF[5], 0, SINE_EASE_IN[5]),
    Rgb888::new(SINE_EASE_IN_HALF[6], 0, SINE_EASE_IN[6]),
    Rgb888::new(SINE_EASE_IN_HALF[7], 0, SINE_EASE_IN[7]),
    Rgb888::new(SINE_EASE_IN_HALF[8], 0, SINE_EASE_IN[8]),
    Rgb888::new(SINE_EASE_IN_HALF[9], 0, SINE_EASE_IN[9]),
    Rgb888::new(SINE_EASE_IN_HALF[10], 0, SINE_EASE_IN[10]),
    Rgb888::new(SINE_EASE_IN_HALF[11], 0, SINE_EASE_IN[11]),
    Rgb888::new(SINE_EASE_IN_HALF[12], 0, SINE_EASE_IN[12]),
    Rgb888::new(SINE_EASE_IN_HALF[13], 0, SINE_EASE_IN[13]),
    Rgb888::new(SINE_EASE_IN_HALF[14], 0, SINE_EASE_IN[14]),
    Rgb888::new(SINE_EASE_IN_HALF[15], SINE_EASE_IN_HALF[15], SINE_EASE_IN[15]),
];

static WHITE_FADE_IN: [Rgb888; 16] = [
    Rgb888::new(SINE_EASE_IN[0], SINE_EASE_IN[0], SINE_EASE_IN[0]),
    Rgb888::new(SINE_EASE_IN[1], SINE_EASE_IN[1], SINE_EASE_IN[1]),
    Rgb888::new(SINE_EASE_IN[2], SINE_EASE_IN[2], SINE_EASE_IN[2]),
    Rgb888::new(SINE_EASE_IN[3], SINE_EASE_IN[3], SINE_EASE_IN[3]),
    Rgb888::new(SINE_EASE_IN[4], SINE_EASE_IN[4], SINE_EASE_IN[4]),
    Rgb888::new(SINE_EASE_IN[5], SINE_EASE_IN[5], SINE_EASE_IN[5]),
    Rgb888::new(SINE_EASE_IN[6], SINE_EASE_IN[6], SINE_EASE_IN[6]),
    Rgb888::new(SINE_EASE_IN[7], SINE_EASE_IN[7], SINE_EASE_IN[7]),
    Rgb888::new(SINE_EASE_IN[8], SINE_EASE_IN[8], SINE_EASE_IN[8]),
    Rgb888::new(SINE_EASE_IN[9], SINE_EASE_IN[9], SINE_EASE_IN[9]),
    Rgb888::new(SINE_EASE_IN[10], SINE_EASE_IN[10], SINE_EASE_IN[10]),
    Rgb888::new(SINE_EASE_IN[11], SINE_EASE_IN[11], SINE_EASE_IN[11]),
    Rgb888::new(SINE_EASE_IN[12], SINE_EASE_IN[12], SINE_EASE_IN[12]),
    Rgb888::new(SINE_EASE_IN[13], SINE_EASE_IN[13], SINE_EASE_IN[13]),
    Rgb888::new(SINE_EASE_IN[14], SINE_EASE_IN[14], SINE_EASE_IN[14]),
    Rgb888::new(SINE_EASE_IN[15], SINE_EASE_IN[15], SINE_EASE_IN[15]),
];

static YELLOW_FADE_IN: [Rgb888; 16] = [
    Rgb888::new(SINE_EASE_IN[0], SINE_EASE_IN[0], 0),
    Rgb888::new(SINE_EASE_IN[1], SINE_EASE_IN[1], 0),
    Rgb888::new(SINE_EASE_IN[2], SINE_EASE_IN[2], 0),
    Rgb888::new(SINE_EASE_IN[3], SINE_EASE_IN[3], 0),
    Rgb888::new(SINE_EASE_IN[4], SINE_EASE_IN[4], 0),
    Rgb888::new(SINE_EASE_IN[5], SINE_EASE_IN[5], 0),
    Rgb888::new(SINE_EASE_IN[6], SINE_EASE_IN[6], 0),
    Rgb888::new(SINE_EASE_IN[7], SINE_EASE_IN[7], 0),
    Rgb888::new(SINE_EASE_IN[8], SINE_EASE_IN[8], 0),
    Rgb888::new(SINE_EASE_IN[9], SINE_EASE_IN[9], 0),
    Rgb888::new(SINE_EASE_IN[10], SINE_EASE_IN[10], 0),
    Rgb888::new(SINE_EASE_IN[11], SINE_EASE_IN[11], 0),
    Rgb888::new(SINE_EASE_IN[12], SINE_EASE_IN[12], 0),
    Rgb888::new(SINE_EASE_IN[13], SINE_EASE_IN[13], 0),
    Rgb888::new(SINE_EASE_IN[14], SINE_EASE_IN[14], 0),
    Rgb888::new(SINE_EASE_IN[15], SINE_EASE_IN[15], SINE_EASE_IN[15]),
];

static YELLOW_ORANGE_FADE_IN: [Rgb888; 16] = [
    Rgb888::new(SINE_EASE_IN[0], SINE_EASE_IN[0] - SINE_EASE_IN_QUARTER[0], 0),
    Rgb888::new(SINE_EASE_IN[1], SINE_EASE_IN[1] - SINE_EASE_IN_QUARTER[1], 0),
    Rgb888::new(SINE_EASE_IN[2], SINE_EASE_IN[2] - SINE_EASE_IN_QUARTER[2], 0),
    Rgb888::new(SINE_EASE_IN[3], SINE_EASE_IN[3] - SINE_EASE_IN_QUARTER[3], 0),
    Rgb888::new(SINE_EASE_IN[4], SINE_EASE_IN[4] - SINE_EASE_IN_QUARTER[4], 0),
    Rgb888::new(SINE_EASE_IN[5], SINE_EASE_IN[5] - SINE_EASE_IN_QUARTER[5], 0),
    Rgb888::new(SINE_EASE_IN[6], SINE_EASE_IN[6] - SINE_EASE_IN_QUARTER[6], 0),
    Rgb888::new(SINE_EASE_IN[7], SINE_EASE_IN[7] - SINE_EASE_IN_QUARTER[7], 0),
    Rgb888::new(SINE_EASE_IN[8], SINE_EASE_IN[8] - SINE_EASE_IN_QUARTER[8], 0),
    Rgb888::new(SINE_EASE_IN[9], SINE_EASE_IN[9] - SINE_EASE_IN_QUARTER[9], 0),
    Rgb888::new(SINE_EASE_IN[10], SINE_EASE_IN[10] - SINE_EASE_IN_QUARTER[10], 0),
    Rgb888::new(SINE_EASE_IN[11], SINE_EASE_IN[11] - SINE_EASE_IN_QUARTER[11], 0),
    Rgb888::new(SINE_EASE_IN[12], SINE_EASE_IN[12] - SINE_EASE_IN_QUARTER[12], 0),
    Rgb888::new(SINE_EASE_IN[13], SINE_EASE_IN[13] - SINE_EASE_IN_QUARTER[13], 0),
    Rgb888::new(SINE_EASE_IN[14], SINE_EASE_IN[14] - SINE_EASE_IN_QUARTER[14], 0),
    Rgb888::new(SINE_EASE_IN[15], SINE_EASE_IN[15] - SINE_EASE_IN_QUARTER[15], SINE_EASE_IN[15] - SINE_EASE_IN_QUARTER[15]),
];