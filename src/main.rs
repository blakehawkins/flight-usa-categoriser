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
    "ABZ" => NotInUsa,
    "AGP" => NotInUsa,
    "AMS" => NotInUsa,
    "ATH" => NotInUsa,
    "BAH" => NotInUsa,
    "BJV" => NotInUsa,
    "BCN" => NotInUsa,
    "BGY" => NotInUsa,
    "BKK" => NotInUsa,
    "BRU" => NotInUsa,
    "BUD" => NotInUsa,
    "CDG" => NotInUsa,
    "CHQ" => NotInUsa,
    "CIA" => NotInUsa,
    "CPH" => NotInUsa,
    "CRL" => NotInUsa,
    "DMK" => NotInUsa,
    "DOH" => NotInUsa,
    "DXB" => NotInUsa,
    "EDI" => NotInUsa,
    "EMA" => NotInUsa,
    "EWR" => InUsa,
    "GLA" => NotInUsa,
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
    "PHX" => InUsa,
    "RIX" => NotInUsa,
    "SAN" => InUsa,
    "SFO" => InUsa,
    "STN" => NotInUsa,
    "SVO" => NotInUsa,
    "TLL" => NotInUsa,
    "TLS" => NotInUsa,
    "TXL" => NotInUsa,
};

fn shorten(airport: &str) -> &str {
    airport.split('(').nth(1).and_then(|part| part.split('/').nth(0)).unwrap_or_else(|| airport)
}

fn main() -> Result<(), std::io::Error> {
    stdinix(|line| {
        let airports = line.trim().split(',').map(str::trim).collect::<Vec<_>>();

        assert_eq!(airports.len(), 2);

        let matches = airports
            .iter()
            .map(|airport| {
                CATEGORISATION
                    .get(airport.trim())
                    .or_else(|| CATEGORISATION.get(shorten(airport)))
                    .oops(&format!("Airport {} not known", airport))
            })
            .collect::<Result<Vec<_>, _>>()?;

        if matches[0] != matches[1] {
            println!("{},{}", shorten(airports[0]), shorten(airports[1]));
        }

        Ok(())
    })
}

#[test]
fn test() {
    assert_eq!(shorten("Los Angeles / Los Angeles International (LAX/KLAX)"), "LAX");
}
