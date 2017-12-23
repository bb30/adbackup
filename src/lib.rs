pub mod devices;
pub mod logging;

extern crate fern;
extern crate chrono;

#[macro_use]
extern crate log;

#[cfg(test)]
#[macro_use]
extern crate hamcrest;

extern crate failure;

pub fn version() -> &'static str {
    "0.1.0"
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
