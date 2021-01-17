use std::{
    env,
    fs
};
use kml::{
    reader::KmlReader,
    types::{Kml, Placemark}
};
use chrono::{DateTime, FixedOffset, Local};
use polars::prelude::*;

fn main() {
    let path = env::args().skip(1).next().expect("Didn't get a path");
    let mut activities = vec![];
    for entry in fs::read_dir(&path).expect("Couldn't read directory") {
        let e = entry.expect("Couldn't read directory");
        let mut k: KmlReader<_, f64> = KmlReader::from_file(e.path()).expect("Couldn't read kml");
        let kml = k.parse().expect("Couldn't parse kml");
        let mut x = unpack(kml);
        activities.append(&mut x);
    }
    activities.sort_by_key(|a| a.start);
    let df = to_dataframe(activities);
    println!("{:?}", df);
}

fn to_dataframe(recs: Vec<ActivityRecord>) -> DataFrame {
    let distance_col = Series::new("distance", 
                                   &recs.iter()
                                        .map(|a| a.distance.clone())
                                        .collect::<Vec<f64>>());
    let activity_col = Series::new("activity",
                                   &recs.iter()
                                        .map(|a| match a.activity {
                                            ActivityKind::Driving => "driving",
                                            ActivityKind::Cycling => "cycling" 
                                        })
                                        .collect::<Vec<_>>());
    let start_arr = Date64Chunked::new_from_naive_datetime("start",
                                                           &recs.iter()
                                                                .map(|a| a.start.with_timezone(&Local).naive_local())
                                                                .collect::<Vec<_>>());
    let end_arr = Date64Chunked::new_from_naive_datetime("end",
                                                         &recs.iter()
                                                              .map(|a| a.end.with_timezone(&Local).naive_local())
                                                              .collect::<Vec<_>>());
    let debt_col = Series::new("debt",
                               &recs.iter()
                                    .scan(0.0, |state, a| {
                                        match a.activity {
                                            ActivityKind::Driving => { *state = *state + a.distance; () },
                                            ActivityKind::Cycling => { *state = *state - a.distance; () },
                                        };
                                        Some(state.clone())
                                    })
                                    .collect::<Vec<_>>());
    DataFrame::new(vec![start_arr.into(),
                        end_arr.into(),
                        distance_col,
                        activity_col,
                        debt_col,
    ]).expect("Couldn't build dataframe")
}

#[derive(Debug, Clone)]
enum ActivityKind {
    Driving,
    Cycling
}

#[derive(Debug)]
struct ActivityRecord {
    start: DateTime<FixedOffset>,
    end: DateTime<FixedOffset>,
    activity: ActivityKind,
    distance: f64
}

impl ActivityRecord {
    fn try_from_placemark(placemark: Placemark) -> Option<ActivityRecord> {
        let ext_data = placemark.children.iter()
            .find(|e| e.name == "ExtendedData")?;
        let category_value = ext_data.children.iter()
            .find(|e| e.attrs.get("name") == Some(&"Category".to_string()))?
            .clone()
            .children
            .into_iter()
            .find(|e| e.name == "value")?
            .content?;
        let distance_value = ext_data.children.iter()
            .find(|e| e.attrs.get("name") == Some(&"Distance".to_string()))?
            .clone()
            .children
            .into_iter()
            .find(|e| e.name == "value")?
            .content?
            .parse::<f64>()
            .expect("Couldn't parse distance");
        let timespan = placemark.children.iter()
            .find(|e| e.name == "TimeSpan")?;
        let start_str = timespan.children.iter()
            .find(|e| e.name == "begin")?
            .content.clone()?;
        let end_str = timespan.children.iter()
            .find(|e| e.name == "end")?
            .content.clone()?;

        let start = DateTime::parse_from_rfc3339(&start_str)
            .expect("Couldn't parse begin time");
        let end = DateTime::parse_from_rfc3339(&end_str)
            .expect("Couldn't parse end time");
        match category_value.as_str() {
            "Driving" => Some(ActivityRecord {
                start: start,
                end: end,
                activity: ActivityKind::Driving,
                distance: distance_value }),
            "Cycling" => Some(ActivityRecord {
                start: start,
                end: end,
                activity: ActivityKind::Cycling,
                distance: distance_value}),
            _ => None
        }
    }
}

fn unpack(kml: Kml<f64>) -> Vec<ActivityRecord> {
    match kml {
        Kml::KmlDocument(kd) => kd.elements.into_iter().map(|e| unpack(e)).flatten().collect(),
        Kml::Document { attrs: _a, elements: e} => e.into_iter().map(|x| unpack(x)).flatten().collect(),
        Kml::Placemark(p) => {
            match ActivityRecord::try_from_placemark(p) {
                Some(a) => vec![a],
                None => vec![]
            }
        },
        _ => vec![]
    }
}
