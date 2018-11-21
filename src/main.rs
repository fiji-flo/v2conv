extern crate v2conv;

use std::env;

use v2conv::app;

fn main() -> Result<(), String> {
    for o in app::run(env::args_os())? {
        println!("{}", o);
    }
    Ok(())
}
