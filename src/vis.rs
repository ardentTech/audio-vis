use embedded_graphics::geometry::Point;
use microfft::Complex32;
use crate::ADC_RESOLUTION;
use crate::color::Color;
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
    fn draw(&self, fb: &mut Fb, samples: [Complex32; T]);
    fn next_color(&mut self);
    fn prev_color(&mut self);
}

#[derive(Debug)]
pub(crate) struct FullHeightVis {
    color: Color
}

impl Default for FullHeightVis {
    fn default() -> Self {
        Self { color: Color::default() }
    }
}

impl MatrixVis<PANEL_WIDTH> for FullHeightVis {

    fn draw(&self, fb: &mut Fb, samples: [Complex32; PANEL_WIDTH]) {
        // TODO can abstract away the next two lines?
        let amplitudes = calc_amplitudes(samples, PANEL_HEIGHT as i32);
        fb.erase();
        for (col, v) in amplitudes.iter().enumerate() {
            for row in 0..*v {
                fb.set_pixel(Point::new(col as i32, (PANEL_HEIGHT as i32) - (*v - row) as i32), self.color.data()[row as usize]);
            }
        }
    }

    fn next_color(&mut self) {
        self.color = self.color.next();
    }

    fn prev_color(&mut self) {
        self.color = self.color.prev();
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

    pub(crate) fn next_color(&mut self) {
        match self {
            Vis::FullHeight(vis) => vis.next_color(),
        }
    }

    pub(crate) fn prev_color(&mut self) {
        match self {
            Vis::FullHeight(vis) => vis.prev_color(),
        }
    }
}