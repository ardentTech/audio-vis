#![no_std]
#![no_main]

mod display;
mod colors;

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
use crate::colors::{BLUE, BLUE_OCEAN, BLUE_VIOLET, CYAN, CYAN_OCEAN, CYAN_TURQUOISE, GREEN, GREEN_SPRING_GREEN, GREEN_TURQUOISE, MAGENTA, MAGENTA_ROSE, OCEAN, ORANGE, ROSE, RED, RED_ORANGE, RED_ROSE, SPRING_GREEN, TURQUOISE, VIOLET, VIOLET_MAGENTA, YELLOW, YELLOW_ORANGE, YELLOW_SPRING_GREEN, ROSE_RED, ORANGE_RED, ORANGE_YELLOW, SPRING_GREEN_YELLOW, SPRING_GREEN_GREEN, ROSE_MAGENTA};
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

// const BRIGHTNESS: u8 = 255;
//const COL_STEPS: u8 = BRIGHTNESS / PANEL_WIDTH as u8;
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

fn draw(fb: &mut Fb, amplitudes: [u32; PANEL_WIDTH]) {
    fb.erase();
    for (col, v) in amplitudes.iter().enumerate() {
        for i in 0..*v {
            fb.set_pixel(Point::new(col as i32, ((PANEL_HEIGHT / 2) - 1) as i32 - i as i32), warm_column(i));
            fb.set_pixel(Point::new(col as i32, (PANEL_HEIGHT / 2) as i32 + i as i32), warm_column(i));
        }
    }
}

fn cool_column(v: u32) -> Rgb888 {
    match v {
        0 => BLUE,
        1 => BLUE_OCEAN,
        2 => OCEAN,
        3 => CYAN_OCEAN,
        4 => CYAN,
        5 => CYAN_TURQUOISE,
        6 => TURQUOISE,
        7 => GREEN_TURQUOISE,
        8 => GREEN,
        9 => GREEN_SPRING_GREEN,
        10 => SPRING_GREEN,
        11 => YELLOW_SPRING_GREEN,
        12 => YELLOW,
        13 => YELLOW_ORANGE,
        14 => ORANGE,
        _ => RED_ORANGE
    }
}

fn warm_column(v: u32) -> Rgb888 {
    match v {
        //0 => ROSE_MAGENTA,
        0 => ROSE,
        1 => ROSE_RED,
        2 => RED_ROSE,
        3 => RED,
        4 => RED_ORANGE,
        5 => ORANGE_RED,
        6 => ORANGE,
        7 => ORANGE_YELLOW,
        8 => YELLOW_ORANGE,
        9 => YELLOW,
        10 => YELLOW_SPRING_GREEN,
        11 => SPRING_GREEN_YELLOW,
        12 => SPRING_GREEN,
        13 => SPRING_GREEN_GREEN,
        14 => GREEN_SPRING_GREEN,
        _ => GREEN,
    }
}

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