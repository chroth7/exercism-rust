#[derive(Debug)]
pub struct Duration(f64);

impl From<u64> for Duration {
    fn from(s: u64) -> Self {
        Duration(s as f64 / 31557600_f64)
    }
}

pub trait Planet {
    fn conversion_factor() -> f64;

    fn years_during(d: &Duration) -> f64 {
        d.0 / Self::conversion_factor()
    }
}

// macro
macro_rules! planet_generation {
    ($n: ident, $c: expr) => {
        pub struct $n;
        impl Planet for $n {
            fn conversion_factor() -> f64 {
                $c
            }
        }
    };
}

planet_generation!(Mercury, 0.2408467);
planet_generation!(Venus, 0.61519726);
planet_generation!(Earth, 1.0);
planet_generation!(Mars, 1.8808158);
planet_generation!(Jupiter, 11.862615);
planet_generation!(Saturn, 29.447498);
planet_generation!(Uranus, 84.016846);
planet_generation!(Neptune, 164.79132);
