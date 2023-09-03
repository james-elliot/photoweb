/*
Dont forget!!!!!!!!!!!!!!!!!!!!!!!
sudo mount -t tmpfs -o size=1g tmpfs /mnt/ramfs

exiftool -geotag=Louisiane.gpx -geosync=+6:00:00 .
*/

#[derive(Debug, serde::Deserialize)]
#[allow(non_snake_case)]
struct CityCsv {
    ASCII_Name: String,
    Country_name: String,
    Coordinates: String,
}

#[derive(Debug)]
struct City {
    city: String,
    country: String,
    lat: f64,
    lon: f64,
}

fn read_cities(file_path: &str) -> Vec<City> {
    let mut tab = Vec::new();
    let file = std::fs::File::open(file_path).unwrap();
    let mut rdr = csv::ReaderBuilder::new().delimiter(b';').from_reader(file);
    for result in rdr.deserialize::<CityCsv>() {
        let r = result.unwrap();
        let v: Vec<&str> = r.Coordinates.split(',').collect();
        let lat = v[0].parse::<f64>().unwrap();
        let lon = v[1].parse::<f64>().unwrap();
        tab.push(City {
            city: r.ASCII_Name,
            country: r.Country_name,
            lat,
            lon,
        });
    }
    tab
}

fn get_latlon(path: &str) -> Option<(f64, f64, String)> {
    let file = std::fs::File::open(path).unwrap();
    let mut bufreader = std::io::BufReader::new(&file);
    let exifreader = exif::Reader::new();
    let exif = exifreader.read_from_container(&mut bufreader).unwrap();
    let mut lat = 0.;
    let mut lon = 0.;
    let mut s = "".to_string();
    for f in exif.fields() {
        if let Some(t) = f.tag.description() {
            //		println!("{:?} {}",t,f.display_value().with_unit(&exif).to_string());
            if t.eq("Latitude") {
                let s = f.display_value().with_unit(&exif).to_string();
                let v: Vec<&str> = s.split(' ').collect();
                lat = v[0].parse::<f64>().unwrap()
                    + v[2].parse::<f64>().unwrap() / 60.
                    + v[4].parse::<f64>().unwrap() / 3600.;
                if v[6].eq("S") {
                    lat = -lat;
                }
            }
            if t.eq("Longitude") {
                let s = f.display_value().with_unit(&exif).to_string();
                let v: Vec<&str> = s.split(' ').collect();
                lon = v[0].parse::<f64>().unwrap()
                    + v[2].parse::<f64>().unwrap() / 60.
                    + v[4].parse::<f64>().unwrap() / 3600.;
                if v[6].eq("W") {
                    lon = -lon;
                }
            }
            if t.eq("Date and time of original data generation") {
                s = f.display_value().with_unit(&exif).to_string();
            }
        }
    }
    if lat == 0. {
        return None;
    };
    Some((lat, lon, s))
}

fn deg2rad(deg: f64) -> f64 {
    deg * std::f64::consts::PI / 180.
}

fn dist(lat1: f64, lon1: f64, lat2: f64, lon2: f64) -> f64 {
    let r = 6371.; // Radius of the earth in km
    let dlat = deg2rad(lat2 - lat1);
    let dlon = deg2rad(lon2 - lon1);
    let a = (dlat / 2.).sin() * (dlat / 2.).sin()
        + (deg2rad(lat1)).cos() * (deg2rad(lat2)).cos() * (dlon / 2.).sin() * (dlon / 2.).sin();
    let c = 2. * a.sqrt().atan2((1. - a).sqrt());
    r * c // Distance in km
}

fn one(p: &std::path::Path, tab: &[City], ext: &str) {
    let _p1 = p.file_stem().and_then(std::ffi::OsStr::to_str);
    let p2 = p.extension().and_then(std::ffi::OsStr::to_str);
    match p2 {
        None => {
            println!("No name?");
            return
        },
        Some(s) => {
            let s2 = s.to_ascii_lowercase();
            if !s2.eq(ext) {
                println!("Bad name? {} {}",s2,ext);
                return;
            }
        }
    }
    let path = p.to_str().unwrap();
    if let Some((lat, lon, date)) = get_latlon(path) {
        let r = tab
            .iter()
            .min_by_key(|x| dist(lat, lon, x.lat, x.lon) as i64)
            .unwrap();
        let v: Vec<&str> = date.split(' ').collect();
        let lab = "label:".to_owned() + v[0] + "\n" + v[1] + "\n" + &r.city + "\n" + &r.country;
        println!("{}", lab);
        let s = "./jpegs/".to_owned() + path;
        let status = std::process::Command::new("/usr/bin/convert")
            .args([
                path,
                "-resize 800x800",
                &s
            ])
            .status()
            .expect("failed to execute process");
        println!("process resize finished with: {status}");
    }
}

fn main() {
    let tab = read_cities("cities.csv");
/*
    for entry in walkdir::WalkDir::new("/mnt/home2/Photos/JMA/M9")
        .into_iter()
        .filter_map(|e| e.ok())
    {
        println!("{}", entry.path().display());
        one(entry.path(), &tab, "dng");
    }
*/

    let path = "./toto.jpg";
    let p = std::path::Path::new(path);
    one(p,&tab,"jpg");

}
