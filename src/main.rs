use cpal::{
    traits::{DeviceTrait, HostTrait, StreamTrait}, Stream
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    
    let (tx, rx) = std::sync::mpsc::channel::<()>();
    let out = start_audio(44100)?;
    out.stream.play()?;

    while let Ok(_) = rx.recv() {}

    Ok(())
}

pub struct OutputStream {
    pub stream: Stream,
}

pub fn start_audio(sample_rate: u32) -> Result<OutputStream, String> {
    const CHANNELS: usize = 1;
    const BUFFER_SIZE: usize = 256;

    let host = cpal::default_host();
    let output_device = host
        .default_output_device()
        .expect("failed to find a default output device");

    let stream_config = cpal::StreamConfig {
        channels: CHANNELS as u16,
        sample_rate: cpal::SampleRate(sample_rate as u32),
        buffer_size: cpal::BufferSize::Fixed(BUFFER_SIZE as u32),
    };

    let mut t = 0_f32;
    let stepsize = 1_f32 / sample_rate as f32;

    let stream = output_device
        .build_output_stream(
            &stream_config,
            move |data: &mut [f32], _info: &cpal::OutputCallbackInfo| {
                if t >= sample_rate as f32 {
                    t -= 1_f32;
                }
                for sample in data.iter_mut() {
                    *sample = (2_f32 * std::f32::consts::PI * 220_f32 * t).sin() * 0.3;
                    t += stepsize;
                }

            },
            move |err| eprintln!("Error on audio output stream: {}", err),
            None,
        )
        .expect("Failed to create input audio stream");

    Ok(OutputStream { stream })
}
