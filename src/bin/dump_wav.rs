use hound;
use num::complex::Complex;
use std::f64::consts::PI;
use std::i16;
use std::thread;

// read_wav はモノラル16bitのwavファイルを読み込み、Vec<f64>に変換して返す関数
fn read_wav(path: &str) -> (hound::WavSpec, Vec<f64>) {
    let mut reader = hound::WavReader::open(path).unwrap();
    let spec = reader.spec();
    let samples = reader
        .samples::<i16>()
        .map(|s| s.unwrap() as f64 / (1 << (spec.bits_per_sample - 1)) as f64) // 正規化
        //.map(|s| s.unwrap() as f64) // 正規化しない
        .collect::<Vec<f64>>();
    (spec, samples)
}

fn dft(signal: &Vec<f64>, dt: f64) -> Vec<Complex<f64>> {
    let n = signal.len();
    let mut spectrum = vec![Complex::new(0.0, 0.0); n];
    let mut handles = vec![];

    for l in 0..n {
        let signal = signal.clone();
        let handle = thread::spawn(move || {
            let mut sum = Complex::new(0.0, 0.0);
            for k in 0..n {
                let wn = -2.0 * PI / n as f64;
                let theta = wn * (k * l) as f64;

                sum += signal[k] * Complex::new(theta.cos(), theta.sin());
            }
            sum * dt
        });
        handles.push(handle);
    }
    for (l, handle) in handles.into_iter().enumerate() {
        spectrum[l] = handle.join().unwrap();
    }

    spectrum
}

fn inverse_dft(spectrum: Vec<Complex<f64>>, dt: f64) -> Vec<f64> {
    let n = spectrum.len();
    let mut signal = vec![0.0; n];
    let mut handles = vec![];

    for l in 0..n {
        let spectrum = spectrum.clone();
        let handle = thread::spawn(move || {
            let mut sum = Complex::new(0.0, 0.0);
            for k in 0..n {
                let wn = 2.0 * PI / n as f64;
                let theta = wn * (k * l) as f64;

                sum += spectrum[k] * Complex::new(theta.cos(), theta.sin());
            }
            sum.re / (n as f64 * dt)
        });
        handles.push(handle);
    }
    for (l, handle) in handles.into_iter().enumerate() {
        signal[l] = handle.join().unwrap();
    }

    signal
}

fn main() {
    let (spec, signal) = read_wav("./sample.wav");
    let dt = 1.0 / spec.sample_rate as f64;
    // t は 0.0, 1/44100, 2/44100, ..., 0.1 となる
    let t = (0..signal.len())
        .map(|i| i as f64 * dt)
        .collect::<Vec<f64>>();
    let spectrum = dft(&signal, dt);
    let invspectrum = inverse_dft(spectrum, dt);

    for i in 0..signal.len() {
        println!("{:.7},{:.7},{:.7}", t[i], signal[i], invspectrum[i]);
    }
}
