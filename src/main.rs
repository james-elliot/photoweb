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
// https://www.google.com/maps/place/1%C2%B021'29.0%22N+103%C2%B059'14.0%22E
fn get_latlon(path: &str) -> Option<(f64, f64, String,String)> {
    let file = std::fs::File::open(path).unwrap();
    let mut bufreader = std::io::BufReader::new(&file);
    let exifreader = exif::Reader::new();
    let exif = exifreader.read_from_container(&mut bufreader).unwrap();
    let mut lat = 0.;
    let mut lon = 0.;
    let mut s = "".to_string();
    let mut g = "https://www.google.com/maps/place/".to_string();
    for f in exif.fields() {
        if let Some(t) = f.tag.description() {
            //		println!("{:?} {}",t,f.display_value().with_unit(&exif).to_string());
            if t.eq("Latitude") {
                let s = f.display_value().with_unit(&exif).to_string();
                let v: Vec<&str> = s.split(' ').collect();
//		eprintln!("{:?}",v);
                lat = v[0].parse::<f64>().unwrap()
                    + v[2].parse::<f64>().unwrap() / 60.
                    + v[4].parse::<f64>().unwrap() / 3600.;
                if v[6].eq("S") {
                    lat = -lat;
                }
		g=g+v[0]+"%C2%B0"+v[2]+"'"+v[4]+"%22"+v[6];
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
		g=g+"+"+v[0]+"%C2%B0"+v[2]+"'"+v[4]+"%22"+v[6];
//		eprintln!("{:?}",g);
            }
            if t.eq("Date and time of original data generation") {
                s = f.display_value().with_unit(&exif).to_string();
            }
        }
    }
    if lat == 0. {
        return None;
    };
    Some((lat, lon, s,g))
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

fn one(p: &std::path::Path, tab: &[City], ext: &str,bl:bool) {
    let _p1 = p.file_stem().and_then(std::ffi::OsStr::to_str);
    let p2 = p.extension().and_then(std::ffi::OsStr::to_str);
    match p2 {
        None => {
            eprintln!("No extension: {:?}",p);
            return
        },
        Some(s) => {
            let s2 = s.to_ascii_lowercase();
            if !s2.eq(ext) {
                eprintln!("Bad name? {:?} {} {}",p,s2,ext);
                return;
            }
        }
    }
    let path = p.to_str().unwrap();
    if let Some((lat, lon, date,g)) = get_latlon(path) {
        let r = tab
            .iter()
            .min_by_key(|x| dist(lat, lon, x.lat, x.lon) as i64)
            .unwrap();
        let v: Vec<&str> = date.split(' ').collect();
        let lab = v[0].to_owned() + ", " + v[1] + ", " + &r.city + ", " + &r.country;
        let s = "./small/".to_owned() + path;
        println!(
r#"<p class="center">
<a href="{path}" target="_blank">
<img src="{s}" alt="" />
</a>
<br/>
{lab}
<a href="{g}" target="_blank">
lat={lat:.6},lon={lon:.6}
</a>
</p>
"#);
	if bl {
            let _status = std::process::Command::new("/usr/bin/convert")
		.args([
                    path,
                    "-resize",
                    "800x800",
                    &s
		])
		.status()
		.expect("failed to execute process");
	}
    }
    else {
	eprintln!("Error : no lat/lon for {path}");
    }
}

fn print_header() {
    println!(
r#"
<?xml version="1.0" encoding= "ISO-8859-1" ?>

<!DOCTYPE html PUBLIC "-//W3C//DTD XHTML 1.0 Transitional//EN"
"http://www.w3.org/TR/xhtml1/DTD/xhtml1-transitional.dtd">



<html xmlns="http://www.w3.org/1999/xhtml" xml:lang="fr" lang="fr">


<head>
<title>

</title>
<link rel="stylesheet" type="text/css" href="/mystyle.css" />
</head>


<body>
<!--#include virtual="/header.shtml.fr" -->

<h1>

</h1>
"#);
}

fn print_footer() {
    println!(
r#"
<!--#include virtual="/footer.shtml.fr" -->
<!-- Local Variables: -->
<!-- coding: latin-1 -->
</body>
</html>
"#);    
}
use argparse::{ArgumentParser, StoreTrue};
fn main() {
    let mut bl = false; 
    { // this block limits scope of borrows by ap.refer() method
        let mut ap = ArgumentParser::new();
        ap.set_description("Build web pages to display a collection of photographs");
        ap.refer(&mut bl)
            .add_option(&["-c","--convert"], StoreTrue,"Also convert images to 800x800 size");
        ap.parse_args_or_exit();
    }
    let tab = read_cities("cities.csv");
    print_header();
    for entry in walkdir::WalkDir::new(".")
	.max_depth(1)
    //	.sort_by(|a,b| a.file_name().cmp(b.file_name()))
	.sort_by_file_name()
        .into_iter()
        .filter_map(|e| e.ok())
    {
        eprintln!("{}", entry.path().display());
        one(entry.path(), &tab, "jpg",bl);
    }
    print_footer();
/*
    let path = "toto.jpg";
    let p = std::path::Path::new(path);
    one(p,&tab,"jpg");
*/
}
