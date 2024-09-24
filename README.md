# Web pages builder for photographs

Creates a web page for a set of photos in a directory, automatically extracting information about location, camera, dates and lenses, and setting up the name of the location (city+country).

The program extracts latitude and longitude, so you need files containing correspondence from lat/lon to location informations. These files can be directly downloaded from Geonames. You need the [allCountries.txt](https://download.geonames.org/export/dump/allCountries.zip) file and the [countryInfo.txt](https://download.geonames.org/export/dump/countryInfo.txt) file

Informations set in the command line take precedence over informations extracted.


    Usage:
      ./target/release/photoweb [OPTIONS]
    
    Build web pages to display a collection of photographs
    
    Optional arguments:
      -h,--help             Show this help message and exit
      -t,--title TITLE      Title of the web page (default: None)
      -e,--extension EXTENSION
                            Extension of files to process (default: jpg)
      -n,--notes NOTES      Name of the file containing notes to add at the start
                            of the webpage (default: None)
      -p,--population POPULATION
                            Minimal number of inhabitants by location (default:
                            1000)
      -g,--geonames GEONAMES
                            Name of geonames file (default: ./allCountries.txt)
      -C,--countryinfo COUNTRYINFO
                            Name of country_info file (default: ./countryInfo.txt)
      -c,--camera CAMERA    Name of camera (default: extracted automatically)
      -l,--lens LENS        Name of lens(es) separated by commas (default:
                            extracted automatically)
      -L,--location LOCATION
                            Location name (default: extracted automatically)
