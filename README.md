#Web pages builder for photographs

Creates a web page for a set of photos in a directory, automatically extracting information about location, camera, dates and lenses.

Locations informations files can be directly downloaded from Geonames. You need the [allCountries.txt](https://download.geonames.org/export/dump/allCountries.zip) file and the [countryInfo.txt](https://download.geonames.org/export/dump/countryInfo.txt) file


    Usage:
     ./target/release/photoweb [OPTIONS]

    Build web pages to display a collection of photographs

    Optional arguments:
      -h,--help             Show this help message and exit
	  -e,--extension EXTENSION
                            Extension of files to process (default jpg)
      -n,--notes NOTES      Name of a file containing notes to add at the start of the webpage
      -p,--population POPULATION
                            Minimal number of inhabitants by location (default 1000)
      -g,--geonamesfile GEONAMESFILE
                            Name of geonames file (default ./allCountries.txt)
      -C,--countryinfo COUNTRYINFO
                            Name of country_info file (default ./countryInfo.txt)
      -c,--camera CAMERA    Name of camera
      -l,--lens LENS        Name of lens(es) separated by commas
      -t,--title TITLE      Title of the web page
      -l,--location LOCATION
                            Location name

