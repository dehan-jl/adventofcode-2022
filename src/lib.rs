use std::env::VarError;
use std::error::Error;
use std::path::{Path, PathBuf};
use std::time::Instant;
use std::{env, fs};
use structopt::StructOpt;

use reqwest::header::COOKIE;

/// This function panics when something goes wrong. That is intended behaviour.
fn get_input(opt: &Opt) -> String {
    let bin = binary_name();
    let day = bin
        .strip_prefix("day")
        .and_then(|b| b.parse::<u8>().ok())
        .unwrap();

    let path = make_path(&bin, opt);
    match (path.exists(), opt.real) {
        (true, _) => fs::read_to_string(path).map_err(|err| Box::new(err) as Box<dyn Error>),
        (false, false) => panic!("Oh no! I couldn't find the example input file :("),
        (false, true) => download_and_save(path, day),
    }
    .unwrap()
}

/// This function may cause a path error when the "inputs/real" directory doesn't exist.
/// The workaround is to create the directory manually.
fn download_and_save(path: PathBuf, day: u8) -> Result<String, Box<dyn Error>> {
    let resp = download_input(2022, day)?;
    match fs::write(path, resp.as_bytes()).map_err(|err| Box::new(err) as Box<dyn Error>) {
        Ok(_) => Ok(resp),
        Err(err) => Err(err),
    }
}

fn binary_name() -> String {
    env::args()
        .next()
        .as_ref()
        .map(Path::new)
        .and_then(Path::file_name)
        .and_then(std::ffi::OsStr::to_str)
        .map(String::from)
        .expect("Couldn't find the binary name for some reason...")
}

fn make_path(bin_name: &str, opt: &Opt) -> PathBuf {
    let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));

    path.push("inputs");
    path.push(if opt.real { "real" } else { "example" });
    path.push(bin_name);
    path.set_extension("txt");

    path
}

fn make_url(year: u16, day: u8) -> String {
    format!("https://adventofcode.com/{year}/day/{day}/input")
}

fn get_session_token() -> Result<String, VarError> {
    env::var("AOC_SESSION")
}

fn download_input(year: u16, day: u8) -> Result<String, Box<dyn Error>> {
    let client = reqwest::blocking::Client::new();
    client
        .get(make_url(year, day))
        .header(
            COOKIE,
            "session=".to_owned() + get_session_token()?.as_str(),
        )
        .send()?
        .text()
        .map_err(|err| Box::new(err) as Box<dyn Error>)
}

pub fn runner(f: impl Fn(&str)) {
    let opt = Opt::from_args();

    let input = get_input(&opt);

    println!("---");
    let start = Instant::now();
    f(&input);
    let duration = start.elapsed();
    println!("--- {duration:?}")
}
#[derive(StructOpt)]
struct Opt {
    #[structopt(short, long)]
    real: bool,
}

pub fn is_real() -> bool {
    let opt = Opt::from_args();
    opt.real
}
