#![no_std]
#![no_main]

mod display;

use defmt::{debug, info, unwrap};
#[allow(unused_imports)]
use {defmt_rtt as _, panic_probe as _};
use embassy_executor::Spawner;
use embassy_rp::adc::{Adc, Async, Channel, Config, InterruptHandler};
use embassy_rp::{bind_interrupts, clocks, pac, pio, Peri};
use embassy_rp::gpio::Pull;
use embassy_rp::interrupt::typelevel::Handler;
use embassy_rp::peripherals::{DMA_CH0, DMA_CH1, PIO0};
use embassy_rp::pio::Pio;
use embassy_sync::blocking_mutex::raw::CriticalSectionRawMutex;
use embassy_sync::channel;
use embassy_sync::signal::Signal;
use embedded_graphics::geometry::Point;
use embedded_graphics::pixelcolor::Rgb888;
use rp_hub75::{on_dma_irq, Hub75Pins16, Hub75, Hub75Config, hub75_descriptors, Hub75Swap};
use crate::display::{Fb, PANEL_CLOCK_HZ, PANEL_HEIGHT, PANEL_WIDTH};

/// The driver's frame-boundary work, in the DMA interrupt.
struct DmaIrqHandler;

impl Handler<embassy_rp::interrupt::typelevel::DMA_IRQ_0> for DmaIrqHandler {
    unsafe fn on_interrupt() {
        on_dma_irq();
    }
}


/// A `'static` place for a value, for the driver's `&'static mut` framebuffers.
macro_rules! mk_static {
    ($t:ty,$val:expr) => {{
        static STATIC_CELL: static_cell::StaticCell<$t> = static_cell::StaticCell::new();
        #[deny(unused_attributes)]
        let x = STATIC_CELL.uninit().write($val);
        x
    }};
}

const ADC_RESOLUTION: i32 = 4096;
const MIC_SAMPLE_BLOCK_SIZE: usize = PANEL_WIDTH * 2;
const READS_PER_SAMPLE: usize = 64;

// TODO this is a command... need trait?
struct ProcessSamples([f32; MIC_SAMPLE_BLOCK_SIZE]);
struct DrawSamples([u32; PANEL_WIDTH]);

enum Event {
    SamplesRead([f32; MIC_SAMPLE_BLOCK_SIZE]),
    SamplesProcessed([u32; PANEL_HEIGHT * 2]),
}

/// Channel for events from worker tasks to the orchestrator
static EVENT_CHANNEL: channel::Channel<CriticalSectionRawMutex, Event, 10> = channel::Channel::new();
static PROCESS_SAMPLES_HANDLER: Signal<CriticalSectionRawMutex, ProcessSamples> = Signal::new();
static DRAW_AMPLITUDES_HANDLER: Signal<CriticalSectionRawMutex, DrawSamples> = Signal::new();


bind_interrupts!(struct Irqs {
    ADC_IRQ_FIFO => InterruptHandler;
    DMA_IRQ_0 => DmaIrqHandler;
    PIO0_IRQ_0 => pio::InterruptHandler<PIO0>;
});

// TODO any harm in using i32s here?
// TODO write unit tests
fn map_range(a: i32, in_min: i32, in_max: i32, out_min: i32, out_max: i32) -> i32 {
    if a < in_min {
        return out_min;
    } else if a > in_max {
        return out_max;
    }
    (a - in_min) * (out_max - out_min) / (in_max - in_min) + out_min
}

#[embassy_executor::task]
async fn samples_read_task(
    mut adc: Adc<'static, Async>,
    mut adc_channel: Channel<'static>,
) {
    debug!("samples_read_task");
    let sender = EVENT_CHANNEL.sender();
    let mut samples = [0.0; MIC_SAMPLE_BLOCK_SIZE];

    loop {
        for i in 0..MIC_SAMPLE_BLOCK_SIZE {
            let mut sum = 0.0f32;
            for _ in 0..READS_PER_SAMPLE {
                sum += adc.read(&mut adc_channel).await.unwrap() as f32;
            }
            samples[i] = sum / READS_PER_SAMPLE as f32;
        }
        sender.send(Event::SamplesRead(samples)).await;
        samples = [0.0; MIC_SAMPLE_BLOCK_SIZE];
    }
}

#[embassy_executor::task]
async fn samples_processing_task() {
    debug!("samples_processing_task");
    let sender = EVENT_CHANNEL.sender();

    loop {
        let mut samples = PROCESS_SAMPLES_HANDLER.wait().await;
        let spectrum = microfft::real::rfft_128(&mut samples.0);
        // since the real-valued coefficient at the Nyquist frequency is packed into the
        // imaginary part of the DC bin, it must be cleared before computing the amplitudes
        //spectrum[0].im = 0.0;
        spectrum[0].re = 0.0;

        let mut amplitudes: [u32; PANEL_WIDTH] = [0; PANEL_WIDTH];
        spectrum.iter().enumerate().for_each(|(i, v)| {
            amplitudes[i] = map_range(v.l1_norm() as i32, 0, ADC_RESOLUTION, 0, (PANEL_HEIGHT / 2) as i32) as u32; // TODO so many type casts
        });
        sender.send(Event::SamplesProcessed(amplitudes)).await;
    }
}

#[embassy_executor::task]
async fn display_task(
    pio: Pio<'static, PIO0>,
    pins: Hub75Pins16,
    dma_ch0: Peri<'static, DMA_CH0>,
    dma_ch1: Peri<'static, DMA_CH1>,
) {
    debug!("display_task");

    let sender = EVENT_CHANNEL.sender();

    // One framebuffer is the panel's at any moment and the other is this loop's;
    // a swap exchanges them.
    let fb0: &'static mut Fb = mk_static!(Fb, Fb::new());
    let fb1: &'static mut Fb = mk_static!(Fb, Fb::new());

    // The framebuffers' addresses, taken before they move into the driver. The
    // alignment does not change the picture; the log is where it is checked.
    let fb0_addr = fb0 as *const Fb as usize;
    let fb1_addr = fb1 as *const Fb as usize;


    let hub75 = Hub75::new(
        pio,
        dma_ch0,
        dma_ch1,
        pins,
        clocks::clk_sys_freq(),
        Hub75Config::new(pac::Interrupt::DMA_IRQ_0).with_panel_clock_hz(PANEL_CLOCK_HZ),
        fb0,
        hub75_descriptors!(Fb),
    )
        .expect("failed to start the panel");

    let mut fb = fb1;

    loop {
        let amplitudes = DRAW_AMPLITUDES_HANDLER.wait().await;
        draw(fb, amplitudes.0);
        match hub75.swap(fb) {
            Ok(mut swap) => {
                swap.wait_for_done().await;
                fb = swap.wait();
            }
            Err((_error, returned)) => fb = returned,
        }
    }
}

// TODO could use u16 instead of u32?
// rainbow
// fn draw(fb: &mut Fb, amplitudes: [u32; PANEL_WIDTH]) {
//     fb.erase();
//
//     for (col, v) in amplitudes.iter().enumerate() {
//         for i in 0..*v {
//             fb.set_pixel(Point::new(col as i32, ((PANEL_HEIGHT / 2) - 1) as i32 - i as i32), color(i));
//             fb.set_pixel(Point::new(col as i32, (PANEL_HEIGHT / 2) as i32 + i as i32), color(i));
//         }
//     }
// }

fn draw(fb: &mut Fb, amplitudes: [u32; PANEL_WIDTH]) {
    fb.erase();

    for (col, v) in amplitudes.iter().enumerate() {
        for i in 0..*v {
            fb.set_pixel(Point::new(col as i32, ((PANEL_HEIGHT / 2) - 1) as i32 - i as i32), color(col, i));
            fb.set_pixel(Point::new(col as i32, (PANEL_HEIGHT / 2) as i32 + i as i32), color(col, i));
        }
    }
}

// cool
fn color(col: usize, v: u32) -> Rgb888 {
    match col {
        0 | 63 => Rgb888::new(128, 0, 128),
        1 | 62 => Rgb888::new(120, 0, 128),
        2 | 61 => Rgb888::new(112, 0, 128),
        3 | 60 => Rgb888::new(104, 0, 128),
        4 | 59 => Rgb888::new(96, 0, 128),
        5 | 58 => Rgb888::new(88, 0, 128),
        6 | 57 => Rgb888::new(80, 0, 128),
        7 | 56 => Rgb888::new(72, 0, 128),
        8 | 55 => Rgb888::new(64, 0, 128),
        9 | 54 => Rgb888::new(56, 0, 128),
        10 | 53 => Rgb888::new(48, 0, 128),
        11 | 52 => Rgb888::new(40, 0, 128),
        12 | 51 => Rgb888::new(32, 0, 128),
        13 | 50 => Rgb888::new(24, 0, 128),
        14 | 49 => Rgb888::new(16, 0, 128),
        15 | 48 => Rgb888::new(8, 0, 128),
        16 | 47 => Rgb888::new(0, 0, 128),
        17 | 46 => Rgb888::new(0, 8, 128),
        18 | 45 => Rgb888::new(0, 16, 128),
        19 | 44 => Rgb888::new(0, 24, 128),
        20 | 43 => Rgb888::new(0, 32, 128),
        21 | 42 => Rgb888::new(0, 40, 128),
        22 | 41 => Rgb888::new(0, 48, 128),
        23 | 40 => Rgb888::new(0, 56, 128),
        24 | 39 => Rgb888::new(0, 64, 128),
        25 | 38 => Rgb888::new(0, 72, 128),
        26 | 37 => Rgb888::new(0, 80, 128),
        27 | 36 => Rgb888::new(0, 88, 128),
        28 | 35 => Rgb888::new(0, 96, 128),
        29 | 34 => Rgb888::new(0, 104, 128),
        30 | 33 => Rgb888::new(0, 112, 128),
        _ => Rgb888::new(0, 120, 128),
    }
}

// fn color(v: u32) -> Rgb888 {
//     let blue = Rgb888::new(0, 0, 128);
//     let ocean = Rgb888::new(0, 64, 128);
//     let cyan = Rgb888::new(0, 128, 128);
//     let turquoise = Rgb888::new(0, 128, 64);
//     let green = Rgb888::new(0, 128, 0);
//     let spring_green = Rgb888::new(64, 128, 0);
//     let yellow = Rgb888::new(128, 128, 0);
//     let orange = Rgb888::new(128, 64, 0);
//     let red = Rgb888::new(128, 0, 0);
//     let raspberry = Rgb888::new(128, 0, 64);
//     let magenta = Rgb888::new(128, 0, 128);
//     let violet = Rgb888::new(64, 0, 128);
//
//     if v < 1 {
//         blue
//     } else if v < 3 {
//         ocean
//     } else if v < 5 {
//         cyan
//     } else if v < 7 {
//         turquoise
//     } else if v < 9 {
//         green
//     } else if v < 11 {
//         spring_green
//     } else if v < 13 {
//         yellow
//     } else if v < 15 {
//         orange
//     } else {
//         red
//     }
// }

#[embassy_executor::task]
async fn orchestration_task() {
    debug!("orchestration_task");
    let receiver = EVENT_CHANNEL.receiver();

    loop {
        match receiver.receive().await {
            Event::SamplesRead(samples) => {
                PROCESS_SAMPLES_HANDLER.signal(ProcessSamples(samples));
            }
            Event::SamplesProcessed(amplitudes) => {
                DRAW_AMPLITUDES_HANDLER.signal(DrawSamples(amplitudes));
            }
        }
    }
}

#[embassy_executor::main]
async fn main(spawner: Spawner) {
    let p = embassy_rp::init(Default::default());

    spawner.spawn(unwrap!(orchestration_task()));

    // The PIO is built first: embassy makes a PIO pin from it.
    let mut pio = Pio::new(p.PIO0, Irqs);
    let pins = Hub75Pins16::new(
        &mut pio, p.PIN_0, p.PIN_1, p.PIN_2, p.PIN_3, p.PIN_4, p.PIN_5, p.PIN_6, p.PIN_7, p.PIN_8,
        p.PIN_9, p.PIN_10,
        // STB and OE are the word's last two signal bits; the clock is not a
        // signal at all — the side-set drives it, and it is wired to GP11.
        p.PIN_12, p.PIN_13, p.PIN_11,
    );

    spawner.spawn(unwrap!(display_task(pio, pins, p.DMA_CH0, p.DMA_CH1)));
    spawner.spawn(unwrap!(samples_processing_task()));

    let adc = Adc::new(p.ADC, Irqs, Config::default());
    let pin = Channel::new_pin(p.PIN_26, Pull::Up);
    spawner.spawn(unwrap!(samples_read_task(adc, pin)));
}