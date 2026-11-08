use embedded_graphics::pixelcolor::Rgb888;

const BRIGHTNESS: u8 = 255;

pub(crate) static RED: Rgb888 = Rgb888::new(BRIGHTNESS, 0, 0);
pub(crate) static RED_ORANGE: Rgb888 = Rgb888::new(BRIGHTNESS, BRIGHTNESS / 4, 0);
pub(crate) static ORANGE: Rgb888 = Rgb888::new(BRIGHTNESS, BRIGHTNESS / 2, 0);
pub(crate) static YELLOW_ORANGE: Rgb888 = Rgb888::new(BRIGHTNESS, BRIGHTNESS - (BRIGHTNESS / 4), 0);
pub(crate) static YELLOW: Rgb888 = Rgb888::new(BRIGHTNESS, BRIGHTNESS, 0);
pub(crate) static YELLOW_SPRING_GREEN: Rgb888 = Rgb888::new(BRIGHTNESS - (BRIGHTNESS / 4), BRIGHTNESS, 0);
pub(crate) static SPRING_GREEN: Rgb888 = Rgb888::new(BRIGHTNESS / 2, BRIGHTNESS, 0);
pub(crate) static GREEN_SPRING_GREEN: Rgb888 = Rgb888::new(BRIGHTNESS / 4, BRIGHTNESS, 0);
pub(crate) static GREEN: Rgb888 = Rgb888::new(0, BRIGHTNESS, 0);
pub(crate) static GREEN_TURQUOISE: Rgb888 = Rgb888::new(0, BRIGHTNESS, BRIGHTNESS / 4);
pub(crate) static TURQUOISE: Rgb888 = Rgb888::new(0, BRIGHTNESS, BRIGHTNESS / 2);
pub(crate) static CYAN_TURQUOISE: Rgb888 = Rgb888::new(0, BRIGHTNESS, BRIGHTNESS - (BRIGHTNESS / 4));
pub(crate) static CYAN: Rgb888 = Rgb888::new(0, BRIGHTNESS, BRIGHTNESS);
pub(crate) static CYAN_OCEAN: Rgb888 = Rgb888::new(0, BRIGHTNESS - (BRIGHTNESS / 4), BRIGHTNESS);
pub(crate) static OCEAN: Rgb888 = Rgb888::new(0, BRIGHTNESS / 2, BRIGHTNESS);
pub(crate) static BLUE_OCEAN: Rgb888 = Rgb888::new(0, BRIGHTNESS / 4, BRIGHTNESS);
pub(crate) static BLUE: Rgb888 = Rgb888::new(0, 0, BRIGHTNESS);
pub(crate) static BLUE_VIOLET: Rgb888 = Rgb888::new(BRIGHTNESS / 4, 0, BRIGHTNESS);
pub(crate) static VIOLET: Rgb888 = Rgb888::new(BRIGHTNESS / 2, 0, BRIGHTNESS);
pub(crate) static VIOLET_MAGENTA: Rgb888 = Rgb888::new(BRIGHTNESS - (BRIGHTNESS / 4), 0, BRIGHTNESS);
pub(crate) static MAGENTA: Rgb888 = Rgb888::new(BRIGHTNESS, 0, BRIGHTNESS);
pub(crate) static MAGENTA_RASPBERRY: Rgb888 = Rgb888::new(BRIGHTNESS, 0, BRIGHTNESS - (BRIGHTNESS / 4));
pub(crate) static RASPBERRY: Rgb888 = Rgb888::new(BRIGHTNESS, 0, BRIGHTNESS / 2);
pub(crate) static RED_RASPBERRY: Rgb888 = Rgb888::new(BRIGHTNESS, 0, BRIGHTNESS / 4);
pub(crate) static WHITE: Rgb888 = Rgb888::new(BRIGHTNESS, BRIGHTNESS, BRIGHTNESS);