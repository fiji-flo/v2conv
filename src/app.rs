use std::ffi::OsString;
use std::path::PathBuf;

use clap::{App, Arg, ArgMatches};
use serde_json;

use hris::map_hris;
use ldap::map_ldap;
use loader::{load_all, Data};
use schema::Profile;

const VERSION: &str = env!("CARGO_PKG_VERSION");

pub fn parse_args<'a, I, T>(itr: I) -> ArgMatches<'a>
where
    I: IntoIterator<Item = T>,
    T: Into<OsString> + Clone,
{
    App::new("v2conv")
        .about("merge them all")
        .version(VERSION)
        .author("Florian Merz <fmerz@mozilla.com>")
        .arg(
            Arg::with_name("hris")
                .short("w")
                .long("hris")
                .takes_value(true)
                .number_of_values(1)
                .required(true)
                .help("hris/workday data"),
        ).arg(
            Arg::with_name("ldap")
                .short("l")
                .long("ldap")
                .takes_value(true)
                .number_of_values(1)
                .required(true)
                .help("ldap data"),
        ).arg(
            Arg::with_name("out")
                .short("o")
                .long("out")
                .takes_value(true)
                .number_of_values(1)
                .help("output file"),
        ).arg(
            Arg::with_name("mozillians")
                .short("m")
                .long("mozillians")
                .takes_value(true)
                .number_of_values(1)
                .required(false)
                .help("mozillians data"),
        ).arg(
            Arg::with_name("avatars_out")
                .short("a")
                .long("avatars_out")
                .takes_value(true)
                .number_of_values(1)
                .help("output dir for avatars"),
        ).arg(
            Arg::with_name("avatars_in")
                .short("i")
                .long("avatars_in")
                .takes_value(true)
                .number_of_values(1)
                .help("input fir for avatars"),
        ).arg(
            Arg::with_name("split")
                .short("s")
                .long("split")
                .takes_value(true)
                .number_of_values(1)
                .help("split output in chunks of s"),
        ).get_matches_from(itr)
}

pub fn run<I, T>(itr: I) -> Result<Vec<String>, String>
where
    I: IntoIterator<Item = T>,
    T: Into<OsString> + Clone,
{
    let matches = parse_args(itr);
    let data = load_all(
        matches.value_of("hris").unwrap(),
        matches.value_of("ldap").unwrap(),
        matches.value_of("mozillians").unwrap_or_default(),
    )?;
    let avatars_in = matches.value_of("avatars_in").map(PathBuf::from);
    let avatars_out = matches.value_of("avatars_out").map(PathBuf::from);
    let profiles: Vec<Profile> = data
        .into_iter()
        .filter_map(|(email, d)| {
            let Data { hris, ldap, mozillians } = d;
            if hris.is_object() && ldap.is_object() {
                let mut p = Profile::default();
                p = map_hris(p, hris);
                match map_ldap(p, ldap, &avatars_in, &avatars_out) {
                    Ok(l) => {
                        p = l;
                    }
                    Err(e) => {
                        eprintln!("{}", e);
                        return None;
                    }
                }
                return Some(p);
            } else {
                if hris.is_object() {
                    eprintln!("no hris for {}", email);
                }
                if ldap.is_object() {
                    eprintln!("no ldap for {}", email);
                }
            }
            None
        }).collect();
    let out = vec![
        serde_json::to_string_pretty(&json!(profiles))
            .map_err(|e| format!("{}", e))?
            .to_owned(),
    ];
    Ok(out)
}
