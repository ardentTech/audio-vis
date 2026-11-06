use rp_hub75::framebuffer::bitplane::plain::DmaFrameBuffer;
use rp_hub75::framebuffer::compute_rows;

/// The panel this example is wired to: 64×64, 1/32 scan.
const PANEL_WIDTH: usize = 64;
pub(crate) const PANEL_HEIGHT: usize = 32;
/// Row pairs: a 1/32-scan panel lights its top half and its bottom half at once.
const NROWS: usize = compute_rows(PANEL_HEIGHT);
/// Six planes: 63 panel scans a frame, where eight flickers.
const PLANES: usize = 6;

pub(crate) const PANEL_CLOCK_HZ: u32 = 10_000_000;

pub(crate) type Fb = DmaFrameBuffer<NROWS, PANEL_WIDTH, PLANES>;



// use embassy_rp::gpio::Output;
// use embassy_sync::blocking_mutex::raw::CriticalSectionRawMutex;
// use embassy_time::Timer;
//
// pub(crate) const DISPLAY_COLUMNS: usize = 16;
// pub(crate) const DISPLAY_ROWS: usize = 4;
//
// pub(crate) static DISPLAY_FRAME_CHANNEL: embassy_sync::channel::Channel<CriticalSectionRawMutex, [u16; DISPLAY_ROWS], 64> = embassy_sync::channel::Channel::new();
//
// pub(crate) struct Display<'a> {
//     buffer: [u16; DISPLAY_ROWS],
//     rows: [Output<'a>; DISPLAY_ROWS],
//     cols: [Output<'a>; DISPLAY_COLUMNS],
// }
//
// impl<'a> Display<'a> {
//     pub(crate) fn new(rows: [Output<'a>; DISPLAY_ROWS], cols: [Output<'a>; DISPLAY_COLUMNS]) -> Self {
//         Self { buffer: [0; DISPLAY_ROWS], rows, cols }
//     }
//
//     // renders the entire matrix by scanning each row top-to-bottom
//     pub(crate) async fn render(&mut self) {
//         for (ri, row) in self.buffer.iter().enumerate() {
//             self.rows[ri].set_high();
//             //Timer::after_nanos(250).await; // TODO why?
//             for i in (0..DISPLAY_COLUMNS).rev() {
//                 if ((row >> i) & 1) == 1 {
//                     self.cols[i].set_high();
//                 } else {
//                     self.cols[i].set_low();
//                 }
//             }
//             Timer::after_millis(4).await; // ~4.16ms for each row (TODO more specific value?)
//             self.rows[ri].set_low();
//             //Timer::after_nanos(250).await; // TODO why?
//         }
//     }
//
//     pub(crate) fn update(&mut self, buffer: [u16; DISPLAY_ROWS]) {
//         self.buffer = buffer;
//     }
// }