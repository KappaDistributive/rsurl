extern crate reqwest;
extern crate clap;
#[macro_use] extern crate serde_derive;

use clap::{App, Arg};

#[derive(Deserialize, Debug)]
struct IS {
    shorturl: String,
}

fn shorten_url(input: &str) -> Result<String, reqwest::Error> {
    let r: String = format!("https://is.gd/create.php?format=json&url={}", input);
    let r: IS = reqwest::get(r.as_str())?.json()?;
    Ok(r.shorturl)
}
fn main() {
    let app = App::new("rsurl")
        .version("0.1.0")
        .author("Stefan Mesken")
        .about("Shortens a URL via is.gd")
        .arg(Arg::with_name("INPUT")
             .help("The URL to shorten")
             .required(true)
             .index(1))
        .get_matches();
    
    match shorten_url(app.value_of("INPUT").unwrap()) {
        Ok(r) => {
            println!("{}", r);
        }
        Err(e) => {
            println!("{:?}", e);
        }
    }
}
