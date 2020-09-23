// https://arxiv.org/pdf/1807.03066.pdf
use rug::{Complex};

fn main() {
    let immirzi = 1.2;
    let two_j1 = 2;
    let two_j2 = 2;
    let two_j3 = 2;
    let two_k1 = 2;
    let two_k2 = 2;
    let two_k3 = 2;
    let two_rho1 = 2.0 * immirzi;
    let two_rho2 = 2.0 * immirzi;
    let two_rho3 = 2.0 * immirzi;

    let value = wigner_3j_sl2c(
        two_j1, two_j2, two_j3,
        two_k1, two_k2, two_k3,
        two_rho1, two_rho2, two_rho3,
    );
    println!("{}", value);
}

fn wigner_3j_sl2c(
    two_j1: i32, two_j2: i32, two_j3: i32,
    two_k1: i32, two_k2: i32, two_k3: i32,
    two_rho1: f32, two_rho2: f32, two_rho3: f32
) -> Complex {
    let d = (two_j3 + 1) as f32;
    let phase = real_negpow(two_j1 -two_j2 +two_j3);
    let chi = chi(two_j1, two_j2, two_j3,
        two_k1, two_k2, two_k3,
        two_rho1, two_rho2, two_rho3);

    let result = phase * d * chi;
    result
}

fn real_negpow(two_j: i32) -> f32 {
    let j = two_j / 2;
    match j % 2 {
        0 => 1.0,
        _ => -1.0,
    }
}

fn chi(
    two_j1: i32, two_j2: i32, two_j3: i32,
    two_k1: i32, two_k2: i32, two_k3: i32,
    two_rho1: f32, two_rho2: f32, two_rho3: f32
) -> Complex {
    let phase = complex_negpow(two_j1 + two_j2 + two_j3 + two_k1 + two_k2 + two_k3) / (4.0 * (2.0*std::f32::consts::PI)).sqrt();
    let d_factor = ((two_j1 + 1) * (two_j2 + 1) * (two_j3 + 1)) as f32;
    // let ln_gamma = ln_gamma(Complex::new(((2 + two_k1 + two_k2 + two_k3)/4) as f32, (two_rho1 + two_rho1 + two_rho1)/4.0);

    // let gamma = ln_gamma;
    let gamma = 1.0;

    let result = phase * gamma * d_factor.sqrt() * kappa();
    result
}

fn complex_negpow(two_j: i32) -> Complex {
    let j = two_j / 2;
    let half_j = j / 2;
    match j % 2 {
        1 => {
            match half_j % 2 {
                0 => Complex::with_val(53, (0.0, 1.0)),
                _ => Complex::with_val(53, (0.0, -1.0)),
            }
        },
        -1 => {
            match half_j % 2 {
                0 => Complex::with_val(53, (0.0, -1.0)),
                _ => Complex::with_val(53, (0.0, 1.0)),
            }
        },
        _ => {
            match half_j % 2 {
                0 => Complex::with_val(53, (1.0, 0.0)),
                _ => Complex::with_val(53, (-1.0, 0.0)),
            }
        },
    }
}

// fn ln_gamma(z: Complex) -> Complex {
//     Complex::new(1.0, 1.0)
// }

fn kappa() -> f32 {
    1.0
}