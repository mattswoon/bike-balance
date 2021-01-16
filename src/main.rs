use std::{
    env,
    fs
};
use kml::{
    reader::KmlReader,
    types::{Kml, Placemark}
};  

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
    println!("{:?}", activities);
}

#[derive(Debug)]
enum Activity {
    Driving(ActivityValue),
    Cycling(ActivityValue)
}


impl Activity {
    fn try_from_placemark(elem: Placemark) -> Option<Activity> {
        let ext_data = elem.children.into_iter()
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
        match category_value.as_str() {
            "Driving" => Some(Activity::Driving(ActivityValue { distance: distance_value })),
            "Cycling" => Some(Activity::Cycling(ActivityValue { distance: distance_value })),
            _ => None
        }
    }
}

#[derive(Debug)]
struct ActivityValue {
    distance: f64,
}

fn unpack(kml: Kml<f64>) -> Vec<Activity> {
    match kml {
        Kml::KmlDocument(kd) => kd.elements.into_iter().map(|e| unpack(e)).flatten().collect(),
        Kml::Document { attrs: _a, elements: e} => e.into_iter().map(|x| unpack(x)).flatten().collect(),
        Kml::Placemark(p) => {
            match Activity::try_from_placemark(p) {
                Some(a) => vec![a],
                None => vec![]
            }
        },
        _ => vec![]
    }
}
