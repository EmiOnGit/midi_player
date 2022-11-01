extern crate synthrs;

use synthrs::sample;
use synthrs::synthesizer::{make_samples_from_midi_file, quantize_samples};
use synthrs::wave;
use synthrs::writer::write_wav_file;

const SAMPLE2: (&str, f64) = ("res/AlesisC2.wav", 65.0);
const SAMPLE_RATE: usize = 16_000;
fn get_wav_sample(index: usize) -> WavSample {
    match index {
        0 | _ => WavSample::new("res/AlesisC2.wav", 44_100, 65.),
    }
}
fn main() {
    write_wav(
        Instrument::Wav(get_wav_sample(0)),
        "res/Canon_in_D.mid",
        "outPiano.wav",
    );
    write_wav(Instrument::Sin, "res/Canon_in_D.mid", "outSin.wav");
    write_wav(Instrument::Square, "res/Canon_in_D.mid", "outSquare.wav");
}

pub fn write_wav(instrument: Instrument, midi: &str, out: &str) {
    let samples: Vec<f64> = match instrument {
        Instrument::Square => {
            make_samples_from_midi_file(wave::square_wave, SAMPLE_RATE, false, midi)
        }
        Instrument::Sin => make_samples_from_midi_file(wave::sine_wave, SAMPLE_RATE, false, midi),
        Instrument::Wav(wav) => {
            let piano_sampler = |frequency: f64| {
                wave::sampler(
                    frequency,
                    &wav.sample,
                    wav.len,
                    wav.frequency,
                    wav.sample_rate,
                )
            };
            make_samples_from_midi_file(piano_sampler, SAMPLE_RATE, false, midi)
        }
        _ => make_samples_from_midi_file(wave::sine_wave, SAMPLE_RATE, false, midi),
    }
    .unwrap();

    write_wav_file(
        &("out/".to_string() + out),
        SAMPLE_RATE,
        &quantize_samples::<i16>(&samples),
    )
    .expect("failed");
}

pub enum Instrument {
    Square,
    Sin,
    Wav(WavSample),
    Other(Box<dyn Fn(f64) -> f64>),
}
pub struct WavSample {
    pub sample: Vec<f64>,
    pub len: usize,
    pub sample_rate: usize,
    pub frequency: f64,
}
impl WavSample {
    pub fn new(path: &'static str, sample_rate: usize, frequency: f64) -> Self {
        let (sample, len) = sample::samples_from_wave_file(path).unwrap();
        WavSample {
            sample,
            len,
            sample_rate,
            frequency,
        }
    }
}
