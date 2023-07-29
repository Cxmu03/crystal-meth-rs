use image::Rgb;
use rand::Rng;

pub struct CellGroup {
    pub color: Rgb<u8>,
    pub spread_rates: [f64; 4]
}

impl CellGroup {
    pub fn random() -> CellGroup {
        let grey_tone = rand::thread_rng().gen_range(1..=255);
        let color: Rgb<u8> = [grey_tone; 3].into();
        let spread_rates: [f64; 4] = [(); 4].map(|_| rand::thread_rng().gen_range(0.3..1.0));

        CellGroup {
            color: color,
            spread_rates: spread_rates
        }
    }
}
