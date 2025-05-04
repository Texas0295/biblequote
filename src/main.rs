mod utils;

use std::io;

use rand::{seq::IndexedRandom, rng};

use utils::*;

fn main() -> Result<(), io::Error> {
    let path_str_opt = get_data_dir_opt();

    if path_str_opt.is_none() {
        eprintln!("[ERROR] cannot get program's data dir");
        return Ok(());
    }

    let mut path_str = path_str_opt.unwrap();
    path_str.push_str("verses.txt");

    let verses = read_verses(path_str)?;

    if verses.is_empty() {
        eprintln!("[INFO] verses.txt is empty");
        return Ok(());
    }

    let mut rng = rng();
    let verse = verses.choose(&mut rng).unwrap();

    println!("{}", verse);

    Ok(())
}
