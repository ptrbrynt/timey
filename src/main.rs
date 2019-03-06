extern crate chrono;
extern crate clap;
use timey;

fn main() {
    match timey::run() {
        Ok(_) => (),
        Err(err) => eprintln!("{}", err),
    }
}
