use scraper::Html;
use std::env;

mod teplota;
mod viac_info;
mod viac_dni;

//"User-Agent", "Mozilla/5.0 (Windows NT 10.0; Win64; rv:86.0) Gecko/20100101 Firefox/86.0"

fn main() {
    if let Err(_) = get_weather() {
        println!("Can't get the weather");
    }
}

fn get_weather() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Error: An argument is required");
        return Ok(());
    }

    let url = format!("https://pocasie.sme.sk/krajina/slovensko/{}", args[1]);
    let resp = attohttpc::get(url).send()?;
    let resp_text = resp.text()?;
    let document = Html::parse_document(resp_text.as_str());

    if let Some(_) = teplota::get(&document) {
        viac_info::get(&document);
        println!();
        viac_dni::get(&document);
    } else {
        println!("Can't get the weather");
    }

    Ok(())
}
