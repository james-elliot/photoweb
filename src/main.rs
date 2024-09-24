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

#[derive(Debug, serde::Deserialize)]
#[allow(non_snake_case)]
#[allow(dead_code)]
struct CountryCsv {
    ISO               : String,
    ISO3              : String,
    ISO_Numeric       : String,
    fips              : String,
    Country           : String,
    Capital           : String,
    Area              : f64,
    Population        : u64,
    Continent         : String,
    tld               : String,
    CurrencyCode      : String,
    CurrencyName      : String,
    Phone             : String,
    Postal_Code_Format: String,
    Postal_Code_Regex : String,
    Languages         : String,
    geonameid         : u64,
    neighbours        : String,
    EquivalentFipsCode: String
}

#[derive(Debug)]
struct Loc {
    location : String,
    country  : String,
    _timezone: String,
    lat      : f64,
    lon      : f64,
}


use std::collections::HashMap;
fn read_locs(path_geo: &str,path_countries: &str,popref:i64) -> Vec<Loc> {
//    let mut countries =  Vec::new();
    let mut codesmap = HashMap::new();
    let file = std::fs::File::open(path_countries).unwrap();
    let mut rdr = csv::ReaderBuilder::new()
        .delimiter(b'\t')
        .comment(Some(b'#'))
        .has_headers(false)
        .from_reader(file);
    for result in rdr.deserialize::<CountryCsv>() {
        let r = result.unwrap();
	codesmap.insert(r.ISO.clone(),r.Country.clone());
//	eprintln!("{:?}",r);
    }
    
    let mut tab = Vec::new();
    let file = std::fs::File::open(path_geo).unwrap();
    let mut rdr = csv::ReaderBuilder::new()
        .delimiter(b'\t')
        .comment(Some(b'#'))
        .has_headers(false)
        .from_reader(file);
    for result in rdr.deserialize::<LocCsv>() {
        let r = result.unwrap();
//        eprintln!("{:?}",r);
        let lat = r.latitude;
        let lon = r.longitude;
        let pop = match r.population {
            Some (n) =>n,
            None => 0};
	let country = match codesmap.get(&r.country_code) {
	    None => "",
	    Some(s) => s};
        if pop>=popref {
            tab.push(Loc {
                location: r.asciiname,
                _timezone: r.timezone,
		country: country.to_string(),
                lat,
                lon,
            });
        }
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
fn one(p: &std::path::Path, tabloc: &[Loc], ext: &str,
       mut fp1: &std::fs::File,
       cam:&String,vlens: &Vec<&str>,locname:&String) {
    let _p1 = p.file_stem().and_then(std::ffi::OsStr::to_str);
    let p2 = p.extension().and_then(std::ffi::OsStr::to_str);
    match p2 {
        None => {
//            eprintln!("No extension: {:?}",p);
            return
        },
        Some(s) => {
            let s2 = s.to_ascii_lowercase();
            if !s2.eq(ext) {
//                eprintln!("Bad name? {:?} {} {}",p,s2,ext);
                return;
            }
        }
    }
    let path = p.to_str().unwrap();
    let (latlon, date,g,cam,exp,fnum,flen,lens,iso,eqlen) = get_latlon(path,cam,vlens);
    let sloc =
	if locname=="" {
	    match latlon {
		Some((lat,lon)) => {
		    let r =
			tabloc.iter()
			.min_by_key(|x| dist(lat, lon, x.lat, x.lon) as i64)
			.unwrap();
		    r.location.clone() + ", " + &r.country
		},
		None => {
		    "".to_string()
		}
	    }
	}
    else {locname.to_string()};
    let v: Vec<&str> = date.split(' ').collect();
    let lab = v[0].to_owned() + ", " + v[1] + ", " + &sloc;
    let (w,h)=image_dimensions(path).expect("Can't get image dimensions");
    let s_medium = "medium/".to_owned()+path;
    let s_small = "small/".to_owned()+path;

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
}



use std::fs::File;
use std::io::Write;
use std::fs;

fn print_header(name:&str,mut fp: &std::fs::File,path:&str) {
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
"#).expect("Can't write header");
    if path!="" {
	match fs::read(path) {
	    Ok(bytes) => fp.write_all(&bytes).expect("Can't write notes"),
	    Err(e) => println!("notes file not present, so not including it: {e:?}"),
	}
    }
}


fn print_footer(mut fp: &std::fs::File) {
    write!(fp,
r#"
       <!--#include virtual="/footer.shtml" -->
    </div>


  </body>
</html>

"#).expect("Can't write footer");    
}


use regex::Regex;
use argparse::{ArgumentParser, Store};
fn main() {
    let mut name = "".to_string();
    let mut cam = "".to_string();
    let mut lens = "".to_string();
    let mut locname = "".to_string();
    let mut locs = "./allCountries.txt".to_string();
    let mut countries = "./countryInfo.txt".to_string();
    let mut notes = "".to_string();
    let mut popref:i64 = 1000;
    { // this block limits scope of borrows by ap.refer() method
        let mut ap = ArgumentParser::new();
        ap.set_description("Build web pages to display a collection of photographs");
        ap.refer(&mut notes)
            .add_option(&["-n","--notes"], Store,
			"Name of a file containing notes to add at the start of the webpage");
        ap.refer(&mut popref)
            .add_option(&["-p","--population"], Store,
			"Minimal number of inhabitants by location (default 1000)");
        ap.refer(&mut locs)
            .add_option(&["-g","--geonamesfile"], Store,
			"Name of geonames file (default ./allCountries.txt)");
        ap.refer(&mut countries)
            .add_option(&["-C","--countryinfo"], Store,
			"Name of country_info file (default ./countryInfo.txt)");
	ap.refer(&mut cam)
            .add_option(&["-c","--camera"], Store,
                        "Name of camera");
	ap.refer(&mut lens)
            .add_option(&["-l","--lens"], Store,
                        "Name of lens(es) separated by commas");
	ap.refer(&mut name)
            .add_option(&["-t","--title"], Store,
                        "Title of the web page");
	ap.refer(&mut locname)
            .add_option(&["-l","--location"], Store,
                        "Location name");
        ap.parse_args_or_exit();
    }
    let vlens: Vec<&str> = lens.split(',').collect();
    let tabloc = read_locs(&locs,&countries,popref); 
    let output_fr = File::create("index.shtml").expect("Can't open index.shtml");
    print_header(&name,&output_fr,&notes);
    for entry in walkdir::WalkDir::new(".")
	.max_depth(1)
    //	.sort_by(|a,b| a.file_name().cmp(b.file_name()))
	.sort_by_file_name()
        .into_iter()
        .filter_map(|e| e.ok())
    {
//        eprintln!("{}", entry.path().display());
        one(entry.path(), &tabloc, "jpg",&output_fr,&cam,&vlens,&locname);
    }
    print_footer(&output_fr);
/*
    let path = "toto.jpg";
    let p = std::path::Path::new(path);
    one(p,&tab,"jpg");
*/
}
