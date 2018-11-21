extern crate clap;
extern crate image;
extern crate rand;
extern crate reqwest;
extern crate serde;
#[macro_use]
extern crate serde_json;
#[macro_use]
extern crate serde_derive;
extern crate uuid;

pub mod app;
mod avatar;
mod hris;
mod ldap;
mod mozillians;
mod loader;
mod schema;