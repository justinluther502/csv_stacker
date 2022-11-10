use polars::prelude::*;
use std::{
    error::Error,
    fs::{read_dir, File},
    io::{prelude::*, BufReader},
};

pub struct Config {
    pub csv_dir_path: String,
    pub colnames: Vec<String>,
    pub outfile: String,
}

impl Config {
    pub fn build(mut args: impl Iterator<Item = String>) -> Result<Config, &'static str> {
        args.next();
        let csv_dir_path = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a CSV directory path argument"),
        };
        let outfile = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get an output file path"),
        };
        let colnames = colnames("colnames.txt");
        Ok(Config {
            csv_dir_path,
            colnames,
            outfile,
        })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let filenames = csv_filenames(&config.csv_dir_path);
    stack_csvs(&filenames, &config.colnames, &config.outfile);
    Ok(())
}

pub fn colnames(filename: &str) -> Vec<String> {
    let file = File::open(filename).expect("couldn't find colnames file.");
    let buf = BufReader::new(file);
    buf.lines()
    .map(|l| l.expect("Could not parse line in colnames file."))
    .collect()
}

pub fn csv_filenames(csv_dir_path: &str) -> Vec<String> {
    read_dir(csv_dir_path)
    .expect("Couln't read CSV directory")
    .map(|f| f.unwrap().path().to_str().unwrap().to_owned())
    .collect()
}

pub fn read_single_csv(filepath: &str) -> Result<DataFrame, PolarsError> {
    Ok(LazyCsvReader::new(filepath)
        .has_header(true)
        .finish()?
        .collect()
        .unwrap())
}

pub fn stack_csvs(csvs: &Vec<String>, colnames: &Vec<String>, out: &str) -> () {
    for csv in csvs {
        println!("{csv}");
    }
    println!("{:?}", colnames);
    println!("{out}");
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
