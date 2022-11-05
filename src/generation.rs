use icalendar::Calendar;
use icalendar::Component;
use icalendar::Event;
use icalendar::Property;
use serde::Deserialize;
use serde::Serialize;
use warp::Filter;
use warp::reject;
use warp::{Reply, reject::Reject};
use warp::filters::BoxedFilter;

use crate::HOST;

#[derive(Debug, Deserialize, Serialize)]
enum CalReject {
    InvalidPeriod(String),
    EndBeforeStart,
}

impl Reject for CalReject {}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct  CalQuery{
    start: u64, // days since epoch
    end: u64, // days since epoch
    participent: String, // Name of the participent
    index:u64, // Index of the participent
    count:u64, // Number of participents
}

// GET /<period>/<chore>?start=<start>&end=<end>&participent=<participent>&index=<index>&count=<count> => 200 OK with the calendar
// Generate a calendar for the given chore for a given participent
// Follows icaleander format
pub fn generate_callendar() -> BoxedFilter<(Box<dyn Reply>,)> {
    warp::path!(String / String)
        .and(warp::query::<CalQuery>())
        .and_then(|periode:String, chore: String, query: CalQuery| async move {
            // Handling input
            let CalQuery{start, end, participent, index, count} = query;
            let period = match periode.as_str() {
                "daily" => 1,
                "weekly" => 7,
                val => match val.parse::<u64>() {
                    Ok(ok) => {ok},
                    Err(_) => {
                        return Err(reject::custom(CalReject::InvalidPeriod(val.to_string())));
                    },
                },
            };

            // If the end is before the start, return an error
            if end < start {
                return Err(reject::custom(CalReject::EndBeforeStart));
            }

            // Compute the days between the start and the end
            let duration = end - start;
            let days = (0..duration).filter(|day|day/period % count == index).map(|day|day+start);
            
            // Create the calendar
            let mut calendar = Calendar::new();

            // Add the url property
            let url = format!("webcal://{HOST}/daily/{chore}?start={start}&end={end}&participent={participent}&index={index}&count={count}");
            calendar.append_property(Property::new("URL", &url));
            // Add the name property
            let name = format!("{chore} for {participent}");
            calendar.append_property(Property::new("NAME", &name));
            
            // Add the events
            for day in days {
                let mut event = Event::new();

                println!("day: {}", day);

                let start = chrono::NaiveDate::from_ymd(1970, 1, 1).and_hms(0, 0, 0) + chrono::Duration::days(day as i64);
                let end = start + chrono::Duration::days(1);

                event.summary(&chore);
                event.starts(start);
                event.ends(end);

                calendar.push(event);
            }

            // Respond with the calendar
            Ok(Box::new(calendar.to_string()) as Box<dyn Reply>)
        })
        .boxed()
}