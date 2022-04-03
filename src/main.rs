#![allow(dead_code)]

extern crate hound;
extern crate rustfft;
extern crate csv;
extern crate statrs;

fn main() {
    generate_bessel_sound();
}

fn generate_bessel_sound() {
    use std::f32::consts::PI;
    use std::i16;
    use hound;
    use statrs::function::factorial::factorial;
    use statrs::function::gamma::gamma;

    let spec = hound::WavSpec {
        channels: 1,
        sample_rate: 44100,
        bits_per_sample: 16,
        sample_format: hound::SampleFormat::Int,
    };

    let mut writer = hound::WavWriter::create("bessel.wav", spec).unwrap();
    let a = 1 as f64;

    for t in (0..44100).map(|x| x as f64 / 44100.0) {
        let mut sample = 0 as f64;

        for m in (0..100) {
            sample = sample + (
                (-1 as f64).powf(m as f64) / factorial(m) / gamma(m as f64 + a + 1.0)
                * (30.0 * t / 2.0).powf(2.0 * m as f64 + a)
            );
        }

        if sample > 1.0 {
            sample = 1.0;
        } else if sample < -1.0 {
            sample = -1.0;
        }

        let guitar = (-2.0 * t as f32).exp() * (
            // Основной тон
            0.7 * (t as f32 * 130.81 * 2.0 * PI).sin() +
            0.53 * (t as f32 * 261.63 * 2.0 * PI).sin() +
            0.3 * (t as f32 * 392.0 * 2.0 * PI).sin() +
            0.053 * (t as f32 * 523.25 * 2.0 * PI).sin() +
            0.03 * (t as f32 * 653.16 * 2.0 * PI).sin() +
            0.012 * (t as f32 * 785.14 * 2.0 * PI).sin() +
            0.037 * (t as f32 * 916.22 * 2.0 * PI).sin()
        ) / 5.55;

        println!("{:?}", sample as f32);

        let amplitude = (i16::MAX as f32);
        writer.write_sample(((sample as f32) * guitar as f32 * amplitude) as i16).unwrap();
    }
}

fn generate_guitar_sound() {
    use std::f32::consts::PI;
    use std::i16;
    use hound;

    let spec = hound::WavSpec {
        channels: 1,
        sample_rate: 44100,
        bits_per_sample: 16,
        sample_format: hound::SampleFormat::Int,
    };

    let mut writer = hound::WavWriter::create("guitar.wav", spec).unwrap();

    for t in (0..44100).map(|x| x as f32 / 44100.0) {
        let sample = (-2.0 * t).exp() * (
            // Main harmonic
            0.7 * (t * 130.81 * 2.0 * PI).sin() +
            0.53 * (t * 261.63 * 2.0 * PI).sin() +
            0.3 * (t * 392.0 * 2.0 * PI).sin() +
            0.053 * (t * 523.25 * 2.0 * PI).sin() +
            0.03 * (t * 653.16 * 2.0 * PI).sin() +
            0.012 * (t * 785.14 * 2.0 * PI).sin() +
            0.037 * (t * 916.22 * 2.0 * PI).sin()
        );

        let amplitude = (i16::MAX as f32) / 5.55;
        writer.write_sample((sample * amplitude) as i16).unwrap();
    }
}

fn draw_spectrum() {
    use hound;
    use rustfft::FFTplanner;
    use rustfft::num_complex::Complex;
    use rustfft::num_traits::Zero;

    let mut reader = hound::WavReader::open("guitar.wav").unwrap();

    let mut input: Vec<Complex<f32>> = reader.samples::<i16>()
        .map(|sample_wrapped| Complex::new(sample_wrapped.unwrap() as f32, 0f32))
        .collect();

    let mut output: Vec<Complex<f32>> = vec![Complex::zero(); 393566];

    let mut planner = FFTplanner::new(false);
    let fft = planner.plan_fft(393566);
    fft.process(&mut input, &mut output);

    let mut writer = csv::Writer::from_path("spectrum-guitar.csv").unwrap();

    for i in 0..output.len() {
        writer.write_record(&[i.to_string(), (output[i].im).to_string()]).unwrap();
    }

    writer.flush().unwrap();
}

fn generate_sine() {
    use std::f32::consts::PI;
    use std::i16;
    use hound;

    let spec = hound::WavSpec {
        channels: 1,
        sample_rate: 44100,
        bits_per_sample: 16,
        sample_format: hound::SampleFormat::Int,
    };

    let mut writer = hound::WavWriter::create("sine.wav", spec).unwrap();

    for t in (0..44100).map(|x| x as f32 / 44100.0) {
        let sample = (
            // Main harmonic
            (t * 110.0 * 2.0 * PI).sin()
        );

        let amplitude = (i16::MAX as f32);
        writer.write_sample((sample * amplitude) as i16).unwrap();
    }
}

fn generate_sound() {
    use std::f32::consts::PI;
    use std::i16;
    use hound;

    let spec = hound::WavSpec {
        channels: 1,
        sample_rate: 44100,
        bits_per_sample: 16,
        sample_format: hound::SampleFormat::Int,
    };

    let mut writer = hound::WavWriter::create("sound.wav", spec).unwrap();

    for t in (0..44100).map(|x| x as f32 / 44100.0) {
        let sample = (-2.0 * t).exp() * (
            // Main harmonic
            (t * 220.0 * 2.0 * PI).sin() +
            (t * 261.63 * 2.0 * PI).sin() +
            (t * 329.63 * 2.0 * PI).sin() +
            // First overtone
            0.5 * (t * 440.0 * 2.0 * PI).sin() +
            0.5 * (t * 523.25 * 2.0 * PI).sin() +
            0.5 * (t * 659.25 * 2.0 * PI).sin() +
            // Second overtone
            0.25 * (t * 660.0 * 2.0 * PI).sin() +
            0.25 * (t * 784.89 * 2.0 * PI).sin() +
            0.25 * (t * 988.89 * 2.0 * PI).sin() +
            // Third overtone
            0.15 * (t * 880.00 * 2.0 * PI).sin() +
            0.15 * (t * 1046.50 * 2.0 * PI).sin() +
            0.15 * (t * 1318.51 * 2.0 * PI).sin() +
            // Fourth overtone
            0.1 * (t * 1100.0 * 2.0 * PI).sin() +
            0.1 * (t * 1308.15 * 2.0 * PI).sin() +
            0.1 * (t * 1648.15 * 2.0 * PI).sin()
        );

        let amplitude = (i16::MAX as f32) / 5.55;
        writer.write_sample((sample * amplitude) as i16).unwrap();
    }
}
