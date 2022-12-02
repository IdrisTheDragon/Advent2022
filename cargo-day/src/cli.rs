use clap::Parser;

///
/// Usage:
/// cargo install --path cargo-day
/// cargo-day 03 
/// 
/// todo fix when calling as `cargo day`
/// cargo day 03
/// error: Found argument '03' which wasn't expected, or isn't valid in this context
/// 
/// (it seems to be picking up 'day' as the day_no arg instead of skipping it.)
/// 
/// 

#[derive(Parser, Debug)]
#[clap(author, version, about)]
pub struct DayArgs {
    /// The day to generate
    day_no: String,

    /// Cargo file to modify
    #[arg(short, long, default_value_t = {"Cargo.toml".to_string()})]
    filename: String,
}

impl DayArgs {
    pub fn day_no(&self) -> &String {
        &self.day_no
    }

    pub fn filename(&self) -> &String {
        &self.filename
    }
}