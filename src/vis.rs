use embedded_graphics::geometry::Point;
use embedded_graphics::pixelcolor::Rgb888;
use microfft::Complex32;
use crate::ADC_RESOLUTION;
use crate::colors::{SPRING_GREEN_FADE_IN_32, WHITE_FADE_IN_32, YELLOW_FADE_IN_32};
use crate::display::{Fb, PANEL_HEIGHT, PANEL_WIDTH};

fn calc_amplitudes(samples: [Complex32; PANEL_WIDTH], out_max: i32) -> [u32; PANEL_WIDTH] {
    let mut amplitudes = [0u32; PANEL_WIDTH];
    samples.iter().enumerate().for_each(|(i, v)| {
        amplitudes[i] = map_range(v.l1_norm() as i32, 0, ADC_RESOLUTION, 0, out_max) as u32; // TODO so many type casts
    });
    amplitudes
}

fn map_range(a: i32, in_min: i32, in_max: i32, out_min: i32, out_max: i32) -> i32 {
    if a < in_min {
        return out_min;
    } else if a > in_max {
        return out_max;
    }
    (a - in_min) * (out_max - out_min) / (in_max - in_min) + out_min
}

trait MatrixVis<const T: usize> {
    fn change_color(&mut self);
    fn draw(&self, fb: &mut Fb, samples: [Complex32; T]);
}

#[derive(Debug)]
pub(crate) struct FullHeightVis {
    colors: [[Rgb888; PANEL_HEIGHT]; 3],
    index: usize
}

impl Default for FullHeightVis {
    fn default() -> Self {
        Self { colors: [SPRING_GREEN_FADE_IN_32, YELLOW_FADE_IN_32, WHITE_FADE_IN_32], index: 0 }
    }
}

impl MatrixVis<PANEL_WIDTH> for FullHeightVis {
    fn change_color(&mut self) {
        self.index = if self.index == 2 { 0 } else { self.index + 1 };
    }

    fn draw(&self, fb: &mut Fb, samples: [Complex32; PANEL_WIDTH]) {
        // TODO can abstract away the next two lines
        let amplitudes = calc_amplitudes(samples, PANEL_HEIGHT as i32);
        fb.erase();
        for (col, v) in amplitudes.iter().enumerate() {
            for row in 0..*v {
                fb.set_pixel(Point::new(col as i32, (PANEL_HEIGHT as i32) - (*v - row) as i32), self.colors[self.index][row as usize]);
            }
        }
    }
}

pub(crate) enum Vis {
    FullHeight(FullHeightVis),
}

impl Vis {
    pub(crate) fn draw(&mut self, fb: &mut Fb, samples: [Complex32; PANEL_WIDTH]) {
        match self {
            Vis::FullHeight(vis) => vis.draw(fb, samples),
        }
    }

    pub(crate) fn change_color(&mut self) {
        match self {
            Vis::FullHeight(vis) => vis.change_color(),
        }
    }
}