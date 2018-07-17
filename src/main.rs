extern crate clap;
extern crate rocksdb;

use clap::{App, Arg};

fn main() {
    let matches = App::new("Geologist")
        .about("RocksDB explorer")
        .arg(Arg::with_name("DB").help("Sets the db").required(true))
        .arg(Arg::with_name("KEY").help("Sets the key").required(true))
        .get_matches();

    let db_path = matches.value_of("DB").unwrap();
    let key = matches.value_of("KEY").unwrap();

    let db = rocksdb::DB::open_default(db_path).expect("error opening db");

    match db.get(key.as_bytes()) {
        Ok(Some(bytes)) => {
            let bytes = bytes.to_vec();
            let s = String::from_utf8_lossy(&bytes);
            println!("{}", s);
        }
        Ok(None) => println!("'{}' not found!", key),
        Err(e) => eprintln!("error {:?}", e),
    }
}
