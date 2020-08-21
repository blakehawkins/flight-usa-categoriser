use oops::Oops;
use phf::phf_map;
use stdinix::stdinix;

#[derive(Debug, PartialEq)]
enum Category {
    InUsa,
    NotInUsa,
}

use crate::Category::*;

static CATEGORISATION: phf::Map<&'static str, Category> = phf_map! {
    "AGP" => NotInUsa,
    "AMS" => NotInUsa,
    "BAH" => NotInUsa,
    "BCN" => NotInUsa,
    "BGY" => NotInUsa,
    "BKK" => NotInUsa,
    "BRU" => NotInUsa,
    "BUD" => NotInUsa,
    "CIA" => NotInUsa,
    "CRL" => NotInUsa,
    "DMK" => NotInUsa,
    "EDI" => NotInUsa,
    "EMA" => NotInUsa,
    "EWR" => InUsa,
    "GRU" => NotInUsa,
    "HEL" => NotInUsa,
    "HKT" => NotInUsa,
    "HND" => NotInUsa,
    "IAD" => InUsa,
    "JFK" => InUsa,
    "KBV" => NotInUsa,
    "KIX" => NotInUsa,
    "LAX" => InUsa,
    "LGW" => NotInUsa,
    "LHR" => NotInUsa,
    "LIS" => NotInUsa,
    "LPX" => NotInUsa,
    "LTN" => NotInUsa,
    "MVD" => NotInUsa,
    "NRT" => NotInUsa,
    "OSL" => NotInUsa,
    "RIX" => NotInUsa,
    "SFO" => InUsa,
    "STN" => NotInUsa,
    "SVO" => NotInUsa,
    "TLS" => NotInUsa,
    "TXL" => NotInUsa,
};

fn main() -> Result<(), std::io::Error> {
    stdinix(|line| {
        let airports = line.trim().split(',').map(str::trim).collect::<Vec<_>>();

        assert_eq!(airports.len(), 2);

        let matches = airports
            .iter()
            .map(|airport| {
                CATEGORISATION
                    .get(airport.trim())
                    .oops(&format!("Airport {} not known", airport))
            })
            .collect::<Result<Vec<_>, _>>()?;

        if matches[0] != matches[1] {
            println!("{},{}", airports[0], airports[1]);
        }

        Ok(())
    })
}
