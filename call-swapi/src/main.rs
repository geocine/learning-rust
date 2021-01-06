#[macro_use]
extern crate clap;

use clap::App;

fn main() {
    // The YAML file is found relative to the current file, similar to how modules are found
    let yaml = load_yaml!("cli.yml");
    let _matches = App::from_yaml(yaml).get_matches();
    // println!("{:#?}", matches);
    let res = login();
    println!("{}", res.unwrap().get("name").unwrap());
}

fn login() -> Result<serde_json::Value, reqwest::Error> {
    let client = reqwest::blocking::Client::new();
    let resp = client
        .get("https://swapi.dev/api/people/1")
        .send()?
        .json::<serde_json::Value>();
    Ok(resp.unwrap())
}
