mod cli;

use std::{
    error::Error,
    fs::{self, File},
    io::Write,
    process::Command,
};

use toml_edit::{value, Document};

use clap::{Parser};
use cli::DayArgs;

fn main() -> Result<(), Box<dyn Error>> {
    // parse args
    let day_args:DayArgs = DayArgs::parse();

    let day = format!("day_{}", day_args.day_no());
    let day_path = day.replace('_', "-");

    // Creat new package from template
    Command::new("cargo")
        .args(["install", "cargo-generate"])
        .output()?;

    Command::new("cargo")
        .args(["generate", "--path", "day_template", "--name", &day])
        .output()?;

    // Add new package to current package workspace and dependencies.
    let content =
        fs::read_to_string(day_args.filename()).map_err(|e| format!("Couldn't open {:?}: {}", day_args.filename(), e))?;
    let mut doc = content.parse::<Document>().expect("invalid doc");
    doc["dependencies"][&day]["path"] = value(&day_path);
    let mut members = doc["workspace"]["members"].as_array().unwrap().clone();
    members.push(&day_path);
    members.fmt();
    doc["workspace"]["members"] = value(members);
    let mut file = File::create("Cargo.toml")?;
    file.write_all(doc.to_string().as_bytes())?;
    
    // todo add calls to library to src/main.rs

    // todo populate input.txt?

    Ok(())
}
