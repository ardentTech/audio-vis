#![no_std]
#![no_main]

use defmt::{debug, info, unwrap};
#[allow(unused_imports)]
use {defmt_rtt as _, panic_probe as _};
use embassy_executor::Spawner;
use embassy_rp::adc::{Adc, Async, Channel, Config, InterruptHandler};
use embassy_rp::{bind_interrupts, dma};
use embassy_rp::gpio::Pull;
use embassy_rp::peripherals::DMA_CH0;
use embassy_sync::blocking_mutex::raw::CriticalSectionRawMutex;
use embassy_sync::channel;
use embassy_sync::signal::Signal;

// TODO this is a command... need trait?
struct ProcessSamples([f32; MIC_SAMPLE_BLOCK_SIZE]);

enum Event {
    SamplesRead([f32; MIC_SAMPLE_BLOCK_SIZE]),
    SamplesProcessed,
}

const MIC_SAMPLE_BLOCK_SIZE: usize = 128;
const MIC_SAMPLE_BATCH_SIZE: usize = 20; // TODO dial this in
const ADC_DIV: u16 = 4799; // 10kHz sample rate (48Mhz / 10kHz - 1)
const DISPLAY_ROWS: usize = 32;

const ADC_LEVELS: i32 = 3250; // TODO dial this in (max is 4096)

/// Channel for events from worker tasks to the orchestrator
static EVENT_CHANNEL: channel::Channel<CriticalSectionRawMutex, Event, 10> = channel::Channel::new();
static PROCESS_SAMPLES_HANDLER: Signal<CriticalSectionRawMutex, ProcessSamples> = Signal::new();


bind_interrupts!(struct Irqs {
    ADC_IRQ_FIFO => InterruptHandler;
    //DMA_IRQ_0 => dma::InterruptHandler<DMA_CH0>;
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
            samples[i] = adc.read(&mut adc_channel).await.unwrap() as f32;
        }
        sender.send(Event::SamplesRead(samples)).await;
        samples = [0.0; MIC_SAMPLE_BLOCK_SIZE];
        // TODO might want to slow this down or avg samples together...
    }
}

#[embassy_executor::task]
async fn samples_processing_task() {
    debug!("samples_processing_task");
    let sender = EVENT_CHANNEL.sender();

    loop {
        let mut samples = PROCESS_SAMPLES_HANDLER.wait().await;
        info!("samples: {}", samples.0);
        let spectrum = microfft::real::rfft_128(&mut samples.0);
        // since the real-valued coefficient at the Nyquist frequency is packed into the
        // imaginary part of the DC bin, it must be cleared before computing the amplitudes
        spectrum[0].im = 0.0;
        spectrum[0].re = 0.0; // TODO not sure about this

        let mut amplitudes: [u32; 64] = [0; 64];
        spectrum.iter().enumerate().for_each(|(i, v)| {
            amplitudes[i] = map_range(v.l1_norm() as i32, 0, ADC_LEVELS, 0, DISPLAY_ROWS as i32) as u32; // TODO so many type casts
        });
        sender.send(Event::SamplesProcessed).await;
    }
}

#[embassy_executor::task]
async fn orchestration_task() {
    debug!("orchestration_task");
    let receiver = EVENT_CHANNEL.receiver();

    loop {
        match receiver.receive().await {
            Event::SamplesRead(samples) => {
                debug!("samples read");
                PROCESS_SAMPLES_HANDLER.signal(ProcessSamples(samples));
            }
            Event::SamplesProcessed => {
                debug!("samples processed");
            }
        }
    }
}

#[embassy_executor::main]
async fn main(spawner: Spawner) {
    let p = embassy_rp::init(Default::default());

    spawner.spawn(unwrap!(orchestration_task()));
    spawner.spawn(unwrap!(samples_processing_task()));

    let adc = Adc::new(p.ADC, Irqs, Config::default());
    let pin = Channel::new_pin(p.PIN_26, Pull::Up);
    spawner.spawn(unwrap!(samples_read_task(adc, pin)));
}