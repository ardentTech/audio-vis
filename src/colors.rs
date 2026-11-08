use embedded_graphics::pixelcolor::Rgb888;

const BRIGHTNESS: u8 = 255;
const STEP: u8 = BRIGHTNESS / 6;

pub(crate) static RED: Rgb888 = Rgb888::new(BRIGHTNESS, 0, 0);
pub(crate) static RED_ORANGE: Rgb888 = Rgb888::new(BRIGHTNESS, STEP, 0);
pub(crate) static ORANGE_RED: Rgb888 = Rgb888::new(BRIGHTNESS, STEP * 2, 0);
pub(crate) static ORANGE: Rgb888 = Rgb888::new(BRIGHTNESS, STEP * 3, 0);
pub(crate) static ORANGE_YELLOW: Rgb888 = Rgb888::new(BRIGHTNESS, STEP * 4, 0);
pub(crate) static YELLOW_ORANGE: Rgb888 = Rgb888::new(BRIGHTNESS, STEP * 5, 0);
pub(crate) static YELLOW: Rgb888 = Rgb888::new(BRIGHTNESS, BRIGHTNESS, 0);

pub(crate) static YELLOW_SPRING_GREEN: Rgb888 = Rgb888::new(STEP * 5, BRIGHTNESS, 0);
pub(crate) static SPRING_GREEN_YELLOW: Rgb888 = Rgb888::new(STEP * 4, BRIGHTNESS, 0);
pub(crate) static SPRING_GREEN: Rgb888 = Rgb888::new(STEP * 3, BRIGHTNESS, 0);
pub(crate) static SPRING_GREEN_GREEN: Rgb888 = Rgb888::new(STEP * 2, BRIGHTNESS, 0);
pub(crate) static GREEN_SPRING_GREEN: Rgb888 = Rgb888::new(STEP, BRIGHTNESS, 0);
pub(crate) static GREEN: Rgb888 = Rgb888::new(0, BRIGHTNESS, 0);
pub(crate) static GREEN_TURQUOISE: Rgb888 = Rgb888::new(0, BRIGHTNESS, STEP);
pub(crate) static TURQUOISE_GREEN: Rgb888 = Rgb888::new(0, BRIGHTNESS, STEP * 2);
pub(crate) static TURQUOISE: Rgb888 = Rgb888::new(0, BRIGHTNESS, STEP * 3);
pub(crate) static TURQUOISE_CYAN: Rgb888 = Rgb888::new(0, BRIGHTNESS, STEP * 4);
pub(crate) static CYAN_TURQUOISE: Rgb888 = Rgb888::new(0, BRIGHTNESS, STEP * 5);
pub(crate) static CYAN: Rgb888 = Rgb888::new(0, BRIGHTNESS, BRIGHTNESS);
pub(crate) static CYAN_OCEAN: Rgb888 = Rgb888::new(0, STEP * 5, BRIGHTNESS);
pub(crate) static OCEAN_CYAN: Rgb888 = Rgb888::new(0, STEP * 4, BRIGHTNESS);
pub(crate) static OCEAN: Rgb888 = Rgb888::new(0, STEP * 3, BRIGHTNESS);
pub(crate) static OCEAN_BLUE: Rgb888 = Rgb888::new(0, STEP * 2, BRIGHTNESS);
pub(crate) static BLUE_OCEAN: Rgb888 = Rgb888::new(0, STEP, BRIGHTNESS);
pub(crate) static BLUE: Rgb888 = Rgb888::new(0, 0, BRIGHTNESS);
pub(crate) static BLUE_VIOLET: Rgb888 = Rgb888::new(STEP, 0, BRIGHTNESS);
pub(crate) static VIOLET_BLUE: Rgb888 = Rgb888::new(STEP * 2, 0, BRIGHTNESS);
pub(crate) static VIOLET: Rgb888 = Rgb888::new(STEP * 3, 0, BRIGHTNESS);
pub(crate) static VIOLET_MAGENTA: Rgb888 = Rgb888::new(STEP * 4, 0, BRIGHTNESS);
pub(crate) static MAGENTA_VIOLET: Rgb888 = Rgb888::new(STEP * 5, 0, BRIGHTNESS);
pub(crate) static MAGENTA: Rgb888 = Rgb888::new(BRIGHTNESS, 0, BRIGHTNESS);
pub(crate) static MAGENTA_ROSE: Rgb888 = Rgb888::new(BRIGHTNESS, 0, STEP * 5);
pub(crate) static ROSE_MAGENTA: Rgb888 = Rgb888::new(BRIGHTNESS, 0, STEP * 4);
pub(crate) static ROSE: Rgb888 = Rgb888::new(BRIGHTNESS, 0, STEP * 3);
pub(crate) static RED_ROSE: Rgb888 = Rgb888::new(BRIGHTNESS, 0, STEP * 2);

pub(crate) static ROSE_RED: Rgb888 = Rgb888::new(BRIGHTNESS, 0, STEP);
pub(crate) static WHITE: Rgb888 = Rgb888::new(BRIGHTNESS, BRIGHTNESS, BRIGHTNESS);