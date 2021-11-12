use std::collections::HashMap;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let resp = reqwest::blocking::get("https://linuxnet.ca/ieee/oui.txt")?
        .json::<HashMap<String, String>>()?;
    println!("{:#?}", resp);
    Ok(())
}
