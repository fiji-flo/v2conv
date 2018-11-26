extern crate v2conv;

use std::env;

use v2conv::app;

fn main() -> Result<(), String> {
    app::run(env::args_os())
}
