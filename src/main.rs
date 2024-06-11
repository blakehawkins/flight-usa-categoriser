use getopt::Opt;
use oops::Oops;
use phf::phf_map;

#[derive(Debug, PartialEq)]
enum Country {
    Usa,
    Uk,
    Other,
}

use crate::Country::*;

static CATEGORISATION: phf::Map<&'static str, Country> = phf_map! {
    "ABZ" => Uk,
    "AGP" => Other,
    "AMS" => Other,
    "ATH" => Other,
    "BAH" => Other,
    "BJV" => Other,
    "BCN" => Other,
    "BGY" => Other,
    "BKK" => Other,
    "BRU" => Other,
    "BUD" => Other,
    "CDG" => Other,
    "CHQ" => Other,
    "CIA" => Other,
    "CPH" => Other,
    "CRL" => Other,
    "DMK" => Other,
    "DIA" => Other,
    "DXB" => Other,
    "EDI" => Uk,
    "EMA" => Uk,
    "EWR" => Usa,
    "GLA" => Uk,
    "GRU" => Other,
    "HEL" => Other,
    "HKT" => Other,
    "HND" => Other,
    "IAD" => Usa,
    "JFK" => Usa,
    "KBV" => Other,
    "KIX" => Other,
    "LAX" => Usa,
    "LGW" => Uk,
    "LHR" => Uk,
    "LIS" => Other,
    "LPA" => Other,
    "LPX" => Other,
    "LTN" => Uk,
    "MAD" => Other,
    "MIA" => Usa,
    "MVD" => Other,
    "NRT" => Other,
    "OLB" => Other,
    "OSL" => Other,
    "PHX" => Usa,
    "RIX" => Other,
    "SAN" => Usa,
    "SFO" => Usa,
    "STN" => Uk,
    "SVO" => Other,
    "TFS" => Other,
    "TLL" => Other,
    "TLS" => Other,
    "TXL" => Other,
};

impl From<&str> for Country {
    fn from(item: &str) -> Country {
        match item {
            "USA" => Country::Usa,
            "UK" => Country::Uk,
            _ => panic!("Unexpected hub country specified"),
        }
    }
}

fn shorten(airport: &str) -> &str {
    airport
        .split('(')
        .nth(1)
        .and_then(|part| part.split('/').next())
        .unwrap_or(airport)
}

struct Opts {
    hub: Country,
    from_field_idx: usize,
    to_field_idx: usize,
}

impl Default for Opts {
    fn default() -> Self {
        Opts {
            hub: Usa,
            from_field_idx: 2,
            to_field_idx: 3,
        }
    }
}

fn main() -> Result<(), std::io::Error> {
    let args = std::env::args().collect::<Vec<_>>();
    let opts: Opts = getopt::Parser::new(&args, "c:f:t:")
        .map(|v| v.expect("failed to parse"))
        .try_fold(Opts::default(), |curr, iter| {
            Ok::<_, core::num::ParseIntError>(match iter {
                Opt('c', Some(hub)) => Opts {
                    hub: hub[..].into(),
                    ..curr
                },
                Opt('f', Some(idx)) => Opts {
                    from_field_idx: idx.parse::<usize>()?,
                    ..curr
                },
                Opt('t', Some(idx)) => Opts {
                    to_field_idx: idx.parse::<usize>()?,
                    ..curr
                },
                _ => unreachable!(),
            })
        })
        .oops("Failed to parse index")?;

    csv::Reader::from_reader(std::io::stdin())
        .records()
        .try_for_each(|line| {
            let line = line.expect("Parsed bad CSV entry");
            let airports = [
                line.get(opts.from_field_idx).unwrap(),
                line.get(opts.to_field_idx).unwrap(),
            ];

            let matches = airports
                .iter()
                .map(|airport| {
                    CATEGORISATION
                        .get(airport.trim())
                        .or_else(|| CATEGORISATION.get(shorten(airport)))
                        .oops(&format!("Airport {} not known", airport))
                })
                .collect::<Result<Vec<_>, _>>()?;

            if matches.iter().any(|v| *v == &opts.hub) && matches.iter().any(|v| *v != &opts.hub) {
                csv::Writer::from_writer(std::io::stdout()).write_record(&line)?;
            }

            Ok::<_, std::io::Error>(())
        })?;

    Ok(())
}

#[test]
fn test() {
    assert_eq!(
        shorten("Los Angeles / Los Angeles International (LAX/KLAX)"),
        "LAX"
    );
}
