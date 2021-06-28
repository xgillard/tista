use std::time::{Duration, UNIX_EPOCH};

use chrono::{DateTime, Utc};
use structopt::StructOpt;

use anyhow::{Context, Result};

/// Prints out the timestamp (u64) for a given date and time
#[derive(StructOpt)]
enum Args {
    /// When you want to convert *to* a timestamp
    ToTista {
        /// The date component of the timestamp in the YYYY-MM-DD format
        date: String,
        /// The time compoent of the timestamp in the HH:mm:SS.ss format
        #[structopt(short, long, default_value="00:00:00.00")]
        time: String
    },
    /// Converts from a timestamp to a human readable date and time
    FromTista {
        /// The timestamp you want to convert as a date time
        tista: u64
    }
}

fn main() -> Result<()> {
    let args = Args::from_args();
    match args {
        Args::ToTista{date, time} => {
            let fmtd = as_rfc3339(&date, &time);
            let date = DateTime::parse_from_rfc3339(&fmtd)
            .with_context(||format!("Error parsing the date {}", fmtd))?
            .with_timezone(&Utc);
            println!("{}", date.timestamp());
        },
        Args::FromTista{tista} => {
            println!("{}", utc(tista))
        }
    }
    Ok(())
}
fn as_rfc3339(date: &str, time: &str) -> String {
    format!("{}T{}Z", date, time)
}
fn utc(tista: u64) -> DateTime<Utc> {
    DateTime::<Utc>::from(UNIX_EPOCH + Duration::from_secs(tista))
        .with_timezone(&Utc)
}
