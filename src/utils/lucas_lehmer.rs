use indicatif::{ProgressBar, ProgressStyle};
use num_bigint::{BigInt, ToBigInt};
use num_traits::{One, Zero, ToPrimitive};
use rustfft::{FftPlanner, num_complex::Complex};
use std::sync::{Arc, Mutex};
use std::thread;

/// FFTベースの高速な乗算
fn fft_mul(x: &BigInt, y: &BigInt) -> BigInt {
    let (x, x_len) = bigint_to_complex_vec(x);
    let (y, y_len) = bigint_to_complex_vec(y);

    let mut planner = FftPlanner::new();
    let fft = planner.plan_fft_forward(x.len());

    let mut x_fft = x.clone();
    let mut y_fft = y.clone();
    fft.process(&mut x_fft);
    fft.process(&mut y_fft);

    let mut z_fft = vec![Complex::zero(); x.len()];
    for i in 0..x.len() {
        z_fft[i] = x_fft[i] * y_fft[i];
    }

    let ifft = planner.plan_fft_inverse(z_fft.len());
    ifft.process(&mut z_fft);

    complex_vec_to_bigint(z_fft, x_len + y_len)
}

/// BigIntをComplex<f64>のベクタに変換する関数
fn bigint_to_complex_vec(x: &BigInt) -> (Vec<Complex<f64>>, usize) {
    let base: u64 = 1 << 32;
    let mut x = x.clone();
    let mut coeffs = Vec::new();
    while x > BigInt::zero() {
        let coeff = (&x % base).to_u64().unwrap();
        coeffs.push(Complex::new(coeff as f64, 0.0));
        x >>= 32;
    }
    let len = coeffs.len();
    while coeffs.len().is_power_of_two() == false {
        coeffs.push(Complex::zero());
    }
    (coeffs, len)
}

/// Complex<f64>のベクタをBigIntに変換する関数
fn complex_vec_to_bigint(coeffs: Vec<Complex<f64>>, len: usize) -> BigInt {
    let base: u64 = 1 << 32;
    let mut result = BigInt::zero();
    let mut coeffs = coeffs;
    coeffs.resize(len, Complex::zero());

    for i in (0..len).rev() {
        result <<= 32;
        result += coeffs[i].re.round() as u64;
    }

    result
}

/// ルーカス-レーマー・テスト
pub fn lucas_lehmer_test(p: usize) -> bool {
    let m = Arc::new(Mutex::new((BigInt::one() << p) - BigInt::one()));
    let s = Arc::new(Mutex::new(4.to_bigint().unwrap()));

    let bar = ProgressBar::new(p as u64);
    bar.set_style(
        ProgressStyle::default_bar()
            .template("{spinner:.green} {msg} [{elapsed_precise}] [{bar:40.cyan/blue}] {pos}/{len} ({eta})")
            .progress_chars("##-"),
    );
    bar.set_message("lucas_lehmer_test");

    let handles: Vec<_> = (2..=p).map(|_| {
        let s = Arc::clone(&s);
        let m = Arc::clone(&m);
        thread::spawn(move || {
            let mut s_lock = s.lock().unwrap();
            let m_lock = m.lock().unwrap();
            *s_lock = fft_mul(&*s_lock, &*s_lock) - 2;
            *s_lock = &*s_lock % &*m_lock;
            *s_lock == BigInt::zero()
        })
    }).collect();

    for handle in handles {
        if handle.join().unwrap() {
            return true;
        }
        bar.inc(1);
    }

    bar.finish_and_clear();
    let s = s.lock().unwrap();
    *s == BigInt::zero()
}

fn main() {
    let p = 31;
    let result = lucas_lehmer_test(p);
    println!("Is 2^{} - 1 a Mersenne prime? {}", p, result);
}
