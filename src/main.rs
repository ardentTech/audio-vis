#![no_std]
#![no_main]

mod display;

use defmt::{info, unwrap};
#[allow(unused_imports)]
use {defmt_rtt as _, panic_probe as _};
use embassy_executor::Spawner;
use embassy_rp::adc::{Adc, Async, Channel, Config, InterruptHandler};
use embassy_rp::{bind_interrupts, dma};
use embassy_rp::gpio::{Level, Output, Pull};
use embassy_rp::peripherals::DMA_CH0;
use embassy_sync::blocking_mutex::raw::CriticalSectionRawMutex;
use embassy_sync::channel::{Receiver, Sender};
use embassy_time::{Duration, Ticker};
use crate::display::{DISPLAY_ROWS, DISPLAY_FRAME_CHANNEL, Display, DISPLAY_COLUMNS};

const MIC_SAMPLE_BLOCK_SIZE: usize = 32;

static MIC_SAMPLES_CHANNEL: embassy_sync::channel::Channel<CriticalSectionRawMutex, [f32; MIC_SAMPLE_BLOCK_SIZE], 64> = embassy_sync::channel::Channel::new();

bind_interrupts!(struct Irqs {
    ADC_IRQ_FIFO => InterruptHandler;
    DMA_IRQ_0 => dma::InterruptHandler<DMA_CH0>;
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
async fn mic_read_task(
    mut adc: Adc<'static, Async>,
    control: Sender<'static, CriticalSectionRawMutex, [f32; MIC_SAMPLE_BLOCK_SIZE], 64>,
    mut dma_channel: dma::Channel<'static>,
    mut adc_channel: Channel<'static>,
) {
    let mut ticker = Ticker::every(Duration::from_millis(100));
    let mut buf = [0_u16; MIC_SAMPLE_BLOCK_SIZE];
    // TODO sample at 2x highest filter frequency to account for Nyquist
    let div = 479; // 100kHz sample rate (48Mhz / 100kHz - 1)
    let mut samples = [0.0; MIC_SAMPLE_BLOCK_SIZE];

    loop {
        adc.read_many(&mut adc_channel, &mut buf, div, &mut dma_channel).await.unwrap();
        buf[..32].iter().enumerate().for_each(|(i, v)| samples[i] = *v as f32);
        info!("samples {:?}", samples);
        control.send(samples).await;
        buf = [0_u16; MIC_SAMPLE_BLOCK_SIZE]; // TODO is this necessary?
        samples = [0.0; MIC_SAMPLE_BLOCK_SIZE]; // TODO is this necessary?
        ticker.next().await;
    }
}

#[embassy_executor::task]
async fn mic_samples_processing_task(
    receiver: Receiver<'static, CriticalSectionRawMutex, [f32; MIC_SAMPLE_BLOCK_SIZE], 64>,
    sender: Sender<'static, CriticalSectionRawMutex, [u16; DISPLAY_ROWS], 64>,
) {
    loop {
        let mut samples = receiver.receive().await;
        let spectrum = microfft::real::rfft_32(&mut samples);
        // since the real-valued coefficient at the Nyquist frequency is packed into the
        // imaginary part of the DC bin, it must be cleared before computing the amplitudes
        spectrum[0].im = 0.0;

        let mut amplitudes: [u32; 16] = [0; 16];
        spectrum.iter().enumerate().for_each(|(i, v)| {
            amplitudes[i] = map_range(v.l1_norm() as i32, 0, 2000, 0, DISPLAY_ROWS as i32) as u32 // TODO so many type casts
        });
        //info!("amplitudes: {:?}", amplitudes);

        let mut frame = [0u16; 4];
        amplitudes.iter().rev().enumerate().for_each(|(i, v)| {
            if *v != 0 {
                for id in (0..*v).rev() {
                    frame[id as usize] += 2u16.pow(i as u32);
                }
            }
        });

        sender.send(frame).await
    }
}

#[embassy_executor::task]
async fn display_task(
    mut display: Display<'static>,
    receiver: Receiver<'static, CriticalSectionRawMutex, [u16; DISPLAY_ROWS], 64>,
) {
    let mut ticker = Ticker::every(Duration::from_hz(60));
    loop {
        display.render().await;
        if !receiver.is_empty() {
            let frame =  receiver.receive().await;
            display.update(frame);
        }
        ticker.next().await;
    }
}

#[embassy_executor::main]
async fn main(spawner: Spawner) {
    let p = embassy_rp::init(Default::default());

    let r0 = Output::new(p.PIN_16, Level::Low);
    let r1 = Output::new(p.PIN_17, Level::Low);
    let r2 = Output::new(p.PIN_18, Level::Low);
    let r3 = Output::new(p.PIN_19, Level::Low);

    let c0 = Output::new(p.PIN_8, Level::Low);
    let c1 = Output::new(p.PIN_9, Level::Low);
    let c2 = Output::new(p.PIN_10, Level::Low);
    let c3 = Output::new(p.PIN_11, Level::Low);
    let c4 = Output::new(p.PIN_12, Level::Low);
    let c5 = Output::new(p.PIN_13, Level::Low);
    let c6 = Output::new(p.PIN_14, Level::Low);
    let c7 = Output::new(p.PIN_15, Level::Low);

    let c8 = Output::new(p.PIN_0, Level::Low);
    let c9 = Output::new(p.PIN_1, Level::Low);
    let c10 = Output::new(p.PIN_2, Level::Low);
    let c11 = Output::new(p.PIN_3, Level::Low);
    let c12 = Output::new(p.PIN_4, Level::Low);
    let c13 = Output::new(p.PIN_5, Level::Low);
    let c14 = Output::new(p.PIN_6, Level::Low);
    let c15 = Output::new(p.PIN_7, Level::Low);

    let rows: [Output; DISPLAY_ROWS] = [r0, r1, r2, r3];
    let columns: [Output; DISPLAY_COLUMNS] = [c0, c1, c2, c3, c4, c5, c6, c7, c8, c9, c10, c11, c12, c13, c14, c15];

    let display = Display::new(rows, columns);

    spawner.spawn(unwrap!(display_task(
        display,
        DISPLAY_FRAME_CHANNEL.receiver(),
    )));

    spawner.spawn(unwrap!(mic_samples_processing_task(
        MIC_SAMPLES_CHANNEL.receiver(),
        DISPLAY_FRAME_CHANNEL.sender(),
    )));

    let adc = Adc::new(p.ADC, Irqs, Config::default());
    let dma_ch = dma::Channel::new(p.DMA_CH0, Irqs);
    let pin = Channel::new_pin(p.PIN_26, Pull::Up);

    spawner.spawn(unwrap!(mic_read_task(
        adc,
        MIC_SAMPLES_CHANNEL.sender(),
        dma_ch,
        pin,
    )));
}