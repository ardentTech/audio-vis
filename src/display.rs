use embassy_rp::gpio::Output;
use embassy_sync::blocking_mutex::raw::CriticalSectionRawMutex;
use embassy_time::Timer;

pub(crate) const DISPLAY_COLUMNS: usize = 16;
pub(crate) const DISPLAY_ROWS: usize = 4;

pub(crate) static DISPLAY_FRAME_CHANNEL: embassy_sync::channel::Channel<CriticalSectionRawMutex, [u16; DISPLAY_ROWS], 64> = embassy_sync::channel::Channel::new();

pub(crate) struct Display<'a> {
    buffer: [u16; DISPLAY_ROWS],
    rows: [Output<'a>; DISPLAY_ROWS],
    cols: [Output<'a>; DISPLAY_COLUMNS],
}

impl<'a> Display<'a> {
    pub(crate) fn new(rows: [Output<'a>; DISPLAY_ROWS], cols: [Output<'a>; DISPLAY_COLUMNS]) -> Self {
        Self { buffer: [0; DISPLAY_ROWS], rows, cols }
    }

    // renders the entire matrix by scanning each row top-to-bottom
    pub(crate) async fn render(&mut self) {
        for (ri, row) in self.buffer.iter().enumerate() {
            self.rows[ri].set_high();
            //Timer::after_nanos(250).await; // TODO why?
            for i in (0..DISPLAY_COLUMNS).rev() {
                if ((row >> i) & 1) == 1 {
                    self.cols[i].set_high();
                } else {
                    self.cols[i].set_low();
                }
            }
            Timer::after_millis(4).await; // ~4.16ms for each row (TODO more specific value?)
            self.rows[ri].set_low();
            //Timer::after_nanos(250).await; // TODO why?
        }
    }

    pub(crate) fn update(&mut self, buffer: [u16; DISPLAY_ROWS]) {
        self.buffer = buffer;
    }
}