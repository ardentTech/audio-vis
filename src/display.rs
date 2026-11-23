use rp_hub75::framebuffer::bitplane::plain::DmaFrameBuffer;
use rp_hub75::framebuffer::compute_rows;

/// The panel this example is wired to: 64×64, 1/32 scan.
pub(crate) const PANEL_WIDTH: usize = 64;
pub(crate) const PANEL_HEIGHT: usize = 32;
/// Row pairs: a 1/32-scan panel lights its top half and its bottom half at once.
const NROWS: usize = compute_rows(PANEL_HEIGHT);
/// Six planes: 63 panel scans a frame, where eight flickers.
const PLANES: usize = 6;

pub(crate) const PANEL_CLOCK_HZ: u32 = 10_000_000;

pub(crate) type Fb = DmaFrameBuffer<NROWS, PANEL_WIDTH, PLANES>;