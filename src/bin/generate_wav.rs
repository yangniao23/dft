use hound;
use std::f64::consts::PI;
use std::i16;
const SAMPLE_RATE: u32 = 44100;

fn write_wav(path: &str, samples: Vec<f64>) {
    let spec = hound::WavSpec {
        channels: 1,
        sample_rate: SAMPLE_RATE as u32,
        bits_per_sample: 16,
        sample_format: hound::SampleFormat::Int,
    };
    let mut writer = hound::WavWriter::create(path, spec).unwrap();
    for s in samples {
        let s = (s * i16::MAX as f64) as i16;
        writer.write_sample(s).unwrap();
    }
}

fn generate_signal(freq: f64, duration: f64) -> Vec<f64> {
    let dt = 1.0 / SAMPLE_RATE as f64;
    let n = (duration / dt) as usize;
    let mut signal = vec![0.0; n];
    for i in 0..n {
        let t = i as f64 * dt;
        signal[i] = (2.0 * PI * freq * t).sin();
    }
    signal
}

fn main() {
    // 1000Hz, 2000Hz, 3000Hzの正弦波を合成した信号を生成する
    let signal = generate_signal(1000.0, 0.1)
        .iter()
        .zip(generate_signal(2000.0, 0.1).iter())
        .zip(generate_signal(3000.0, 0.1).iter())
        .map(|((&s1, &s2), &s3)| s1 + 2.0 * s2 + 3.0 * s3)
        .collect::<Vec<f64>>();

    // 信号の正規化
    let abs_max = signal
        .iter()
        .fold(f64::MIN, |max, &s| f64::max(max, s.abs()));
    let signal = signal.iter().map(|s| s / abs_max).collect::<Vec<f64>>();

    //let signal = generate_signal(1000.0, 0.1);

    write_wav("./sample.wav", signal);
}
