use chrono::{DateTime, Utc};

fn main() {
    for s in [
        "1970-01-01T00:00:00.030Z",
        "1970-01-01T00:00:00.030+0000",
        "1970-01-01T00:00:00.030+00:00",
    ] {
        if let Ok(dt) = DateTime::parse_from_str(s, "%+") {
            println!("Parsed epoch millis are: {}", dt.timestamp_millis())
        } else {
            eprintln!("Couldn't parse {s} with %+")
        }
        if let Ok(dt) = s.parse::<DateTime<Utc>>() {
            println!("Parsed epoch millis are: {}", dt.timestamp_millis())
        } else {
            eprintln!("Couldn't parse {s} as DateTime<Utc>")
        }
    }
}
