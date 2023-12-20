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

fn dft(signal: Vec<f64>, dt: f64) -> Vec<Complex<f64>> {
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

                sum += signal[k] * Complex::new(theta.cos(), theta.sin()) * dt;
            }
            sum
            //sum * dt
        });
        handles.push(handle);
    }
    for (l, handle) in handles.into_iter().enumerate() {
        spectrum[l] = handle.join().unwrap();
    }

    spectrum
}

fn main() {
    let (spec, signal) = read_wav("./sample.wav");
    let len: usize = signal.len();
    let dt = 1.0 / spec.sample_rate as f64;
    let f = (0..(signal.len() / 2))
        .map(|i| i as f64 * spec.sample_rate as f64 / len as f64)
        .collect::<Vec<f64>>();
    let spectrum = dft(signal, dt);

    // スペクトルの最大値を1に正規化
    let abs_max = spectrum
        .iter()
        .fold(f64::MIN, |max, &s| f64::max(max, s.norm()));
    let spectrum = spectrum
        .iter()
        .map(|s| s / abs_max)
        .collect::<Vec<Complex<f64>>>();

    for i in 0..(len / 2) {
        //println!("{:.7},{:.7}", f[i], 2.0 * spectrum[i].norm() / len as f64);
        println!("{:.7},{:.7}", f[i], spectrum[i].norm());
    }
}

/*
fn main() {
    let signal = (0..1000)
        .map(|i| (i as f64 * 2.0 * PI / 1000.0).sin())
        .collect::<Vec<f64>>();
    let f = (0..500)
        .map(|i| i as f64 * 1000.0 / signal.len() as f64)
        .collect::<Vec<f64>>();
    let spectrum = dft(signal, 0.001);

    for i in 0..500 {
        println!("{:.7},{:.7}", f[i], 2.0 * spectrum[i].norm());
    }
}
*/
