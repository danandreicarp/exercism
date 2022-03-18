// The code below is a stub. Just enough to satisfy the compiler.
// In order to pass the tests you can add-to or change any of this code.

#[macro_export]
macro_rules! a_planet {
    ($name: ty, $years: literal) => {
        impl Planet for $name {
            fn years_during(d: &Duration) -> f64 {
                let seconds_in_year = ($years as f64 * 31557600_f64);
                println!("here one year has {} seconds", seconds_in_year);
                d.seconds / seconds_in_year
            }
        }
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

pub struct Mercury;
pub struct Venus;
pub struct Earth;
pub struct Mars;
pub struct Jupiter;
pub struct Saturn;
pub struct Uranus;
pub struct Neptune;

a_planet![Mercury, 0.2408467];
a_planet![Venus, 0.61519726];
a_planet![Earth, 1];
a_planet![Mars, 1.8808158];
a_planet![Jupiter, 11.862615];
a_planet![Saturn, 29.447498];
a_planet![Uranus, 84.016846];
a_planet![Neptune, 164.79132];
