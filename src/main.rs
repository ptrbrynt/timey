extern crate chrono;
extern crate clap;
extern crate clipboard;
use timey;

fn main() {
    match timey::run() {
        Ok(_) => (),
        Err(err) => eprintln!("{}", err),
    }
}
