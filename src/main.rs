use chrono::{DateTime, Utc};
use structopt::StructOpt;

use anyhow::{Context, Result};

/// Prints out the timestamp (u64) for a given date and time
#[derive(StructOpt)]
struct Args {
    /// The date component of the timestamp in the YYYY-MM-DD format
    date: String,
    /// The time compoent of the timestamp in the HH:mm:SS.ss format
    #[structopt(short, long, default_value="00:00:00.00")]
    time: String,
}

fn main() -> Result<()> {
    let args = Args::from_args();
    let date = DateTime::parse_from_rfc3339(&args.as_rfc3339())
                .with_context(||format!("Error parsing the date {}", args.as_rfc3339()))?
                .with_timezone(&Utc);
    println!("{}", date.timestamp());
    Ok(())
}

impl Args {
    fn as_rfc3339(&self) -> String {
        format!("{}T{}Z", &self.date, &self.time)
    }
}
