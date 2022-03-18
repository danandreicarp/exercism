// The code below is a stub. Just enough to satisfy the compiler.
// In order to pass the tests you can add-to or change any of this code.

#[macro_export]
macro_rules! planets {
    ( $(($name: ident, $years: literal)),+) => {
        $(
            #[derive(Debug)]
            pub struct $name;

            impl Planet for $name {
                fn years_during(d: &Duration) -> f64 {
                    let seconds_in_year = ($years as f64 * 31557600_f64);
                    println!("on {:?} one year has {} seconds", $name, seconds_in_year);
                    d.seconds / seconds_in_year
                }
            }
        )*
    };
}

#[derive(Debug)]
pub struct Duration {
    seconds: f64,
}

impl From<u64> for Duration {
    fn from(s: u64) -> Self {
        Duration { seconds: s as f64 }
    }
}

pub trait Planet {
    fn years_during(d: &Duration) -> f64 {
        unimplemented!(
            "convert a duration ({:?}) to the number of years on this planet for that duration",
            d,
        );
    }
}

planets![
    (Mercury, 0.2408467),
    (Venus, 0.61519726),
    (Earth, 1),
    (Mars, 1.8808158),
    (Jupiter, 11.862615),
    (Saturn, 29.447498),
    (Uranus, 84.016846),
    (Neptune, 164.79132)
];
