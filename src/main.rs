use std::io::Read;

use phf::phf_map;

#[derive(Debug, PartialEq)]
enum Category {
    InUsa,
    NotInUsa
}

static CATEGORISATION: phf::Map<&'static str, Category> = phf_map! {
    "AGP" => Category::NotInUsa,
    "AMS" => Category::NotInUsa,
    "BAH" => Category::NotInUsa,
    "BCN" => Category::NotInUsa,
    "BGY" => Category::NotInUsa,
    "BKK" => Category::NotInUsa,
    "BRU" => Category::NotInUsa,
    "BUD" => Category::NotInUsa,
    "CIA" => Category::NotInUsa,
    "CRL" => Category::NotInUsa,
    "DMK" => Category::NotInUsa,
    "EDI" => Category::NotInUsa,
    "EMA" => Category::NotInUsa,
    "EWR" => Category::InUsa,
    "GRU" => Category::NotInUsa,
    "HEL" => Category::NotInUsa,
    "HKT" => Category::NotInUsa,
    "HND" => Category::NotInUsa,
    "JFK" => Category::InUsa,
    "KBV" => Category::NotInUsa,
    "KIX" => Category::NotInUsa,
    "LAX" => Category::InUsa,
    "LGW" => Category::NotInUsa,
    "LHR" => Category::NotInUsa,
    "LIS" => Category::NotInUsa,
    "LPX" => Category::NotInUsa,
    "LTN" => Category::NotInUsa,
    "MVD" => Category::NotInUsa,
    "NRT" => Category::NotInUsa,
    "OSL" => Category::NotInUsa,
    "RIX" => Category::NotInUsa,
    "SFO" => Category::InUsa,
    "STN" => Category::NotInUsa,
    "SVO" => Category::NotInUsa,
    "TLS" => Category::NotInUsa,
    "TXL" => Category::NotInUsa,
};

fn main() -> Result<(), std::io::Error> {
    let mut buf = String::new();

    std::io::stdin().lock().read_to_string(&mut buf)?;

    buf.trim().split('\n').for_each(|line| {
        let mut matches = line
            .trim()
            .split(',')
            .map(|st| (st, CATEGORISATION.get(st)))
            .filter(|opt| opt.1.is_some())
            .map(|pair| (pair.0, pair.1.unwrap()));

        if matches.clone().count() != 2 {
            println!("!! {:?}", matches);
        }

        let pair = (matches.next().unwrap(), matches.next().unwrap());

        if (pair.0).1 != (pair.1).1 {
            println!("{}\t{}", (pair.0).0, (pair.1).0);
        }
    });

    Ok(())
}
