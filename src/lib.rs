use polars::prelude::*;
use std::{
    fs::{read_dir, File},
    io::{prelude::*, BufReader},
};

pub fn read_single_csv(filepath: &str) -> Result<DataFrame, PolarsError> {
    Ok(LazyCsvReader::new(filepath)
        .has_header(true)
        .finish()?
        .collect()
        .unwrap())
}

pub fn get_colnames(filename: &str) -> Vec<String> {
    let file = File::open(filename).expect("couldn't find colnames file.");
    let buf = BufReader::new(file);
    buf.lines()
        .map(|l| l.expect("Could not parse line in colnames file."))
        .collect()
}

pub fn get_csv_filenames(csv_dir_path: &str) -> Vec<String> {
    read_dir(csv_dir_path)
        .expect("Couln't read CSV directory")
        .map(|f| f.unwrap().path().to_str().unwrap().to_owned())
        .collect()
}

pub fn stack_csvs(csvs: &Vec<String>, colnames: &Vec<String>, out: &str) -> () {
    for csv in csvs {
        println!("{}", csv);
    }
    println!("{:?}", colnames);
    println!("{}", out);
}

#[cfg(test)]
mod tests {
    //    use super::*;

    //    #[test]
    //    fn it_works() {
    //        let result = add(2, 2);
    //        assert_eq!(result, 4);
    //    }
}
