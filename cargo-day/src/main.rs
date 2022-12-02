mod cli;

use std::{
    error::Error,
    fs::{self, File},
    io::Write,
    process::Command,
};

use toml_edit::{value, Document};

use clap::Parser;
use cli::DayArgs;

fn main() -> Result<(), Box<dyn Error>> {
    // parse args
    let day_args: DayArgs = DayArgs::parse();

    let day = format!("day_{}", day_args.day_no());
    let day_path = day.replace('_', "-");

    // Creat new package from template
    Command::new("cargo")
        .args(["install", "cargo-generate"])
        .output()?;

    Command::new("cargo")
        .args(["generate", "--path", day_args.template(), "--name", &day])
        .output()?;

    // Add new package to current package workspace and dependencies.
    let content = fs::read_to_string(day_args.cargoname())
        .map_err(|e| format!("Couldn't open {:?}: {}", day_args.cargoname(), e))?;
    let mut doc = content.parse::<Document>().expect("invalid doc");
    doc["dependencies"][&day]["path"] = value(&day_path);
    let mut members = doc["workspace"]["members"].as_array().unwrap().clone();
    members.push(&day_path);
    members.fmt();
    doc["workspace"]["members"] = value(members);
    let mut file = File::create(day_args.cargoname())?;
    file.write_all(doc.to_string().as_bytes())?;

    // add calls to library to src/main.rs
    let mut lib_call = "let x = day_{no}::solve(\"day-{no}/input.txt\")?;
    println!(\"Day {no} Part1: {}\", x.0);
    println!(\"Day {no} Part2: {}\", x.1);
    
    //{{next day}}"
        .to_string();
    let mut lib_import = "use day_{no}::day_{no};\n//{{next day import}}".to_string();
    let mut main_rs = fs::read_to_string(day_args.mainname())
        .map_err(|e| format!("Couldn't open {:?}: {}", day_args.mainname(), e))?;

    lib_call = lib_call.replace("{no}", day_args.day_no());
    lib_import = lib_import.replace("{no}", day_args.day_no());

    main_rs = main_rs.replace("//{{next day}}", &lib_call);
    main_rs = main_rs.replace("//{{next day import}}", &lib_import);

    let mut file = File::create(day_args.mainname())?;
    file.write_all(main_rs.to_string().as_bytes())?;

    // todo populate input.txt?

    Ok(())
}
