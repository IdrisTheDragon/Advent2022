use std::{fs::{self, File}, error::Error, io::Write, process::Command};

use toml_edit::{Document, value};

use std::env;

fn main() -> Result<(), Box<dyn Error>> {
    let filename = "Cargo.toml";
    let content = fs::read_to_string(filename).map_err(|e| format!("Couldn't open {}: {}",filename,e))?;
    
    let mut doc = content.parse::<Document>().expect("invalid doc");

    let mut args = env::args();
    args.next();
    let day = args.next().ok_or("No day value")?;

    
    doc["dependencies"][format!("day_{}",day)]["path"] = value(format!("day_{}",day));

    let mut members = doc["workspace"]["members"].as_array().unwrap().clone();
    members.push(format!("day_{}",day));
    members.fmt();
    doc["workspace"]["members"] = value(members);
 
    println!("{}", doc.to_string());

    let mut file = File::create("Cargo.toml")?;
    file.write_all(doc.to_string().as_bytes())?;

    Command::new("cargo")
        .args(["new","--lib",&format!("day_{}",day)])
        .output()?;

    Ok(())
}
