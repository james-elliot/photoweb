#[derive(Debug, serde::Deserialize)]
#[allow(non_snake_case)]
#[allow(dead_code)]
struct LocCsv {
    geonameid         : u64, //integer id of record in geonames database
    name              : String, //name of geographical point (utf8) varchar(200)
    asciiname         : String, //name of geographical point in plain ascii characters, varchar(200)
    alternatenames    : String, //alternatenames, comma separated, ascii names automatically transliterated, convenience attribute from alternatename table, varchar(10000)
    latitude          : f64, // latitude in decimal degrees (wgs84)
    longitude         : f64, // longitude in decimal degrees (wgs84)
    feature_class     : String, //see http://www.geonames.org/export/codes.html, char(1)
    feature_code      : String, //see http://www.geonames.org/export/codes.html, varchar(10)
    country_code      : String, //ISO-3166 2-letter country code, 2 characters
    cc2               : String, //alternate country codes, comma separated, ISO-3166 2-letter country code, 200 characters
    admin1_code       : String, //fipscode (subject to change to iso code), see exceptions below, see file admin1Codes.txt for display names of this code; varchar(20)
    admin2_code       : String, //code for the second administrative division, a county in the US, see file admin2Codes.txt; varchar(80) 
    admin3_code       : String, //code for third level administrative division, varchar(20)
    admin4_code       : String, //code for fourth level administrative division, varchar(20)
    population        : Option<i64>, //bigint (8 byte int) 
    elevation         : Option<i64>, //in meters, integer
    dem               : String, //digital elevation model, srtm3 or gtopo30, average elevation of 3''x3'' (ca 90mx90m) or 30''x30'' (ca 900mx900m) area in meters, integer. srtm processed by cgiar/ciat.
    timezone          : String, //the iana timezone id (see file timeZone.txt) varchar(40)
    modification_date : String // date of last modification in yyyy-MM-dd format
}

#[derive(Debug)]
struct Loc {
    location: String,
    _timezone: String,
    lat: f64,
    lon: f64,
}

fn read_locs(file_path: &str) -> Vec<Loc> {
    let mut tab = Vec::new();
    let file = std::fs::File::open(file_path).unwrap();
    let mut rdr = csv::ReaderBuilder::new().delimiter(b'\t').from_reader(file);
    for result in rdr.deserialize::<LocCsv>() {
        let r = result.unwrap();
        let lat = r.latitude;
        let lon = r.longitude;
        tab.push(Loc {
            location: r.asciiname,
            _timezone: r.timezone,
            lat,
            lon,
        });
    }
    tab
}

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
fn get_latlon(path: &str,cam:&String,vlens:&Vec<&str>)
              -> (Option<(f64, f64)>, String,String,String,String,String,String,String,String,String) {
    let file = std::fs::File::open(path).unwrap();
    let mut bufreader = std::io::BufReader::new(&file);
    let exifreader = exif::Reader::new();
    let exif = exifreader.read_from_container(&mut bufreader).unwrap();
    let (mut lat,mut lon) = (0.,0.);
//    let (mut s,mut cam,mut exp,mut fnum,mut flen,mut lens) : (String,String,String,String,String,String);
    let (mut s,mut exp,mut fnum,mut flen,mut iso,mut lens,mut eqlen) =
	("".to_string(),"".to_string(),"".to_string(),"".to_string(),"".to_string(),"".to_string(),"".to_string());
    let mut cam = cam.to_owned();
    let mut g = "https://www.google.com/maps/place/".to_string();
    for f in exif.fields() {
//	eprintln!("tag={} ifd_nm={} value={} description={:?}", f.tag, f.ifd_num, f.display_value(),f.tag.description());
        if let Some(t) = f.tag.description() {
//            eprintln!("{:?} {}",t,f.display_value().with_unit(&exif).to_string());
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
            if t.eq("Model of image input equipment") {
                cam = f.display_value().with_unit(&exif).to_string();
		if cam.len()>2 {
		    cam.pop();cam.remove(0);
		}
            }
            if t.eq("Exposure time") {
                exp = f.display_value().with_unit(&exif).to_string();
            }
            if t.eq("F number") {
                let tmp = f.display_value().with_unit(&exif).to_string();
		if tmp.len()>=6 {fnum = tmp[..6].to_string();}
		else {fnum=tmp;}
            }
            if t.eq("Lens focal length") {
                flen = f.display_value().with_unit(&exif).to_string();
            }
            if t.eq("Focal length in 35 mm film") {
		eqlen=f.display_value().with_unit(&exif).to_string();
            }
	    
            if lens.is_empty() &&(t.eq("Lens model") || t.eq("Lens Model")) {
                lens = f.display_value().with_unit(&exif).to_string();
		if lens.len()>2 {
		    lens.pop();lens.remove(0);
		}
	    }
            if t.eq("Photographic sensitivity") {
                iso = f.display_value().with_unit(&exif).to_string();
            }
        }
    }
    if (lens.is_empty()) && (!flen.is_empty()) {
	println!("lens={} flen={}",lens,flen);
	let re = Regex::new(r"(?<fl>[0-9]*)").unwrap();
	let fl_num:i32 = match re.captures(&flen) {
	    Some(caps) => {caps["fl"].parse().unwrap()},
	    None => {panic!("can't get flen");}
	};
	match vlens.len() {
	    0 => {},
	    1 => {lens = vlens[0].to_owned();},
	    _ => {
		let re = Regex::new(r"(?<low>[0-9]*)[ ]*-[ ]*(?<high>[0-9]*)").unwrap();
		for hay in vlens {
		    /*
		    match re.captures(hay) {
			Some(caps) => {
			    let low: i32 = caps["low"].parse().unwrap();
			    let high: i32 = caps["high"].parse().unwrap();
			    println!("{} {}",low,high);
			    if (fl_num>=low) && (fl_num<=high) {
				lens=hay.to_string();
				break;
			    }
			},
			None => {}
		    }
		     */
		    if let Some(caps) = re.captures(hay) {
			let low: i32 = caps["low"].parse().unwrap();
			let high: i32 = caps["high"].parse().unwrap();
			println!("{} {}",low,high);
			if (fl_num>=low) && (fl_num<=high) {
			    lens=hay.to_string();
			    break;
			}
                    }
		    
		}
	    }
	}
    }
    if lat == 0. {
        return (None,s,g,cam,exp,fnum,flen,lens,iso,eqlen)
    };
    return (Some ((lat,lon)),s,g,cam,exp,fnum,flen,lens,iso,eqlen)
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

use image::image_dimensions as image_dimensions;
fn one(p: &std::path::Path, tab: &[City], tabloc: &Option<Vec<Loc>>, ext: &str,do_s:bool,
       do_m:bool,mut fp1: &std::fs::File,cam:&String,vlens: &Vec<&str>,
       output_dirs:&str,output_dirm:&str) {
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
    let (latlon, date,g,cam,exp,fnum,flen,lens,iso,eqlen) = get_latlon(path,cam,vlens);
    let sloc = match latlon {
	Some((lat,lon)) => {
	    let r = tab
		.iter()
		.min_by_key(|x| dist(lat, lon, x.lat, x.lon) as i64)
		.unwrap();
	    let rloc = tabloc.as_ref().map(|s|
					   s.iter()
					   .min_by_key(|x| dist(lat, lon, x.lat, x.lon) as i64)
					   .unwrap());
	    let rloc = rloc.map_or("".to_string(),|s| s.location.to_string()+", ");
	    rloc + &r.city + ", " + &r.country
	},
	None => {
	    "".to_string()
	}
    };
    let v: Vec<&str> = date.split(' ').collect();
    let lab = v[0].to_owned() + ", " + v[1] + ", " + &sloc;
    let s_small = output_dirs.to_owned() + "/" + path;
    let s_medium = output_dirm.to_owned() + "/" + path;
    let (w,h)=image_dimensions(path).expect("Can't get image dimensions");
    
    write!(fp1,
r#"<p class="center">
<a href="{s_medium}" target="_blank">
<img src="{s_small}" alt="" />
</a>
<br/>
{lab}
"#).expect("Can't write to file");

    match latlon {
	Some((lat,lon)) => {
	    write!(fp1,
r#"<a href="{g}" target="_blank">
lat={lat:.6}, lon={lon:.6}
</a>
<br/>
"#) .expect("Can't write to file");
	},
	None => {}
    };

   write!(fp1,
r#"{cam}, {lens}, {fnum}, {exp}, {flen}"#).expect("Can't write to file");
    if !eqlen.is_empty() {
	write!(fp1," (35mm equ: {eqlen})").expect("Can't write to file");
    }

    write!(fp1,
r#", {iso} ISO, {w}x{h}
</p>
"#).expect("Can't write to file");

    if do_s {
	let status = std::process::Command::new("/usr/bin/convert")
	    .args([
                "-resize",
                "800x800",
                &path,
                &s_small,
	    ])
	    .status()
	    .expect("failed to execute process convert");
        if !status.success() {
	    eprintln!("process convert finished with status {} for file {:?}",status,p);
	    return;
	}
    }
    if do_m {
	let status = std::process::Command::new("/usr/bin/convert")
	    .args([
                "-resize",
                "3000x3000",
                &path,
                &s_medium,
	    ])
	    .status()
	    .expect("failed to execute process convert");
        if !status.success() {
	    eprintln!("process convert finished with status {} for file {:?}",status,p);
	    return;
	}
    }
}



use std::fs::File;
use std::io::Write;
use std::fs;

fn print_french_header(name:&str,mut fp: &std::fs::File) {
    write!(fp,
r#"<!DOCTYPE html>
<html lang="fr">
  <head>
    <meta charset="utf-8">
    <link rel="stylesheet" type="text/css" href="/mystyle.css" >
    <link rel="shortcut icon" type="image/x-icon" href="/favicon.bmp" >

    <title>Photo-lovers
    </title>
    <meta name="Category" content="Photographie" >
    <meta name="Description" content="Tout sur la photographie numérique" >
    <meta name="Author" content="Jean-Marc Alliot" >
    <meta name="Keywords" content="Photographie,Numérique" >
  </head>


  <body>

    <nav class="sidenav">
      <iframe class="sidenav" src="/toc.html">
      </iframe>
    </nav>


    <div class="main">
      <!--#include virtual="/header.shtml" -->

<h1>
{name}
</h1>
<p>
En cliquant sur une image, elle s'ouvrira dans un autre onglet en 3000x2000.
<br/>
Toutes les images sont copyrightees (voir le bas de page) et marquees par steganographie.
</p>
"#).expect("Can't write french header");
    match fs::read("notes-fr.txt") {
	Ok(bytes) => fp.write_all(&bytes).expect("Can't write french notes"),
	Err(e) => println!("error reading french notes: {e:?}"),
    }
}


fn print_french_footer(mut fp: &std::fs::File) {
    write!(fp,
r#"
       <!--#include virtual="/footer.shtml" -->
    </div>


  </body>
</html>

"#).expect("Can't write french footer");    
}


use regex::Regex;
use argparse::{ArgumentParser, StoreTrue,Store};
fn main() {
    let mut do_s = false;
    let mut do_m = false;
    let mut name = "".to_string();
    let mut cam = "".to_string();
    let mut lens = "".to_string();
    let mut cities = "./cities.csv".to_string();
    let mut do_g = false;
    let mut locs = "./locs.csv".to_string();
    let mut output_dirs = "./small".to_string();
    let mut output_dirm = "./medium".to_string();
    { // this block limits scope of borrows by ap.refer() method
        let mut ap = ArgumentParser::new();
        ap.set_description("Build web pages to display a collection of photographs");
	ap.refer(&mut do_g)
            .add_option(&["-g","--geonames"], StoreTrue,
			"Use also full geonames file");
        ap.refer(&mut locs)
            .add_option(&["-G","--geonamesfile"], Store,
			"Name of geonames file (default ./locs.csv)");
	ap.refer(&mut cam)
            .add_option(&["-C","--camera"], Store,
                        "Name of camera");
	ap.refer(&mut lens)
            .add_option(&["-L","--lens"], Store,
                        "Name of lens(es) separated by commas");
	ap.refer(&mut name)
            .add_option(&["-t","--title"], Store,
                        "Title of the web page");
	ap.refer(&mut cities)
            .add_option(&["-c","--cities"], Store,
			"Path of file holding cities names (default ./cities.csv)");
	ap.refer(&mut do_s)
            .add_option(&["-s","--small"], StoreTrue,
			"Also create images of 800x800 size");
	ap.refer(&mut output_dirs)
            .add_option(&["-S","--smalldir"], Store,
			"Name of output directory for 800x800 images (default ./small)");
	ap.refer(&mut do_m)
            .add_option(&["-m","--medium"], StoreTrue,
			"Also create images of 3000x3000 size");
	ap.refer(&mut output_dirm)
            .add_option(&["-M","--mediumdir"], Store,
			"Name of output directory for 3000x3000 images (default ./medium)");
        ap.parse_args_or_exit();
    }
    let vlens: Vec<&str> = lens.split(',').collect();
    let tab = read_cities(&cities);
    let tabloc = if  do_g {let v = read_locs(&locs); Some(v)} else {None};
    let output_fr = File::create("index.shtml").expect("Can't open index.shtml");
    print_french_header(&name,&output_fr);
    for entry in walkdir::WalkDir::new(".")
	.max_depth(1)
    //	.sort_by(|a,b| a.file_name().cmp(b.file_name()))
	.sort_by_file_name()
        .into_iter()
        .filter_map(|e| e.ok())
    {
        eprintln!("{}", entry.path().display());
        one(entry.path(), &tab, &tabloc, "jpg",do_s,do_m,&output_fr,&cam,&vlens,&output_dirs,&output_dirm);
    }
    print_french_footer(&output_fr);
/*
    let path = "toto.jpg";
    let p = std::path::Path::new(path);
    one(p,&tab,"jpg");
*/
}
