//! Quick utility to vertically stack a bunch of CSV files with partially
//! matching column headers.
//!
//! Can be configured by editing colnames.txt to specify which common column
//! headers to select from, as well as passing two mandatory arguments:
//! 1. Path of directory with individual CSVs. Ex: csvs
//! 2. Output filename. Ex: combined_csv.csv

use polars::prelude::*;
use std::{
    error::Error,
    fs::{read_dir, read_to_string, File},
    io::{prelude::*, BufReader},
    process,
    path::Path,
};
use serde_derive::Deserialize;
use toml;

/// Config struct representing a few config variables for the app.
#[derive(Deserialize)]
pub struct Config {
    /// The directory holding CSVs to be stacked.
    pub csv_dir_path: String,
    /// Columns to select for the stacked CSV output.
    pub colnames: Vec<String>,
    /// The output csv filename.
    pub outfile: String,
}

impl Config {
    /// Returns the Config struct.
    ///
    /// # Arguments
    ///
    /// * `args` - An iterator from std::env::args with the binary arguments as Items.
    pub fn build() -> Result<Config, &'static str> {
        let contents = read_to_string("Config.toml").expect("Couldn't read Config.toml");
        let config: Config = toml::from_str(&contents).unwrap_or_else(|err| {
            eprintln!("Couldn't parse Config.toml: {err}");
            process::exit(1);
        });
        Ok(config)
    }
}

/// Main function for the crate binary.
///
/// Takes the Config struct, stacks the CSVs, and writes out to the output file
/// defined in the Config struct.
pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let filenames = csv_filenames(&config.csv_dir_path);
    let dfs = build_df_vec(filenames);
    let mut df = stack_dfs(dfs, &config.colnames).unwrap();
    println!("{df}");

    let filename = config.outfile;
    let mut file = std::fs::File::create(&filename).unwrap();
    CsvWriter::new(&mut file)
        .finish(&mut df)
        .unwrap_or_else(|err| {
            eprintln!("Failed to write to file: {filename}, with error: {err}.");
            process::exit(1);
        });
    println!("Saved combined CSV file as {filename}");
    Ok(())
}

/// Takes a Vec of csv filename strings and returns a Vec of LazyFrames.
pub fn build_df_vec(csv_filenames: Vec<String>) -> Vec<LazyFrame> {
    let mut dfs: Vec<LazyFrame> = Vec::new();
    for csv in csv_filenames {
        let df = read_single_csv(&csv).unwrap_or_else(|err| {
            eprintln!("Problem reading from csv file: {err}");
            process::exit(1);
        });
        dfs.push(df)
    }
    dfs
}

/// Reads a plain text file with lines containing the column names desired to
/// be selected for the final stacked CSV.
pub fn colnames(filename: &str) -> Vec<String> {
    let file = File::open(filename).expect("couldn't find colnames file.");
    let buf = BufReader::new(file);
    buf.lines()
        .map(|l| l.expect("Could not parse line in colnames file."))
        .collect()
}

/// Lists all CSV files in the csv_dir_path argument directory and returns the
/// list as a Vec.
pub fn csv_filenames(csv_dir_path: &str) -> Vec<String> {
    let path = Path::new(csv_dir_path);
    read_dir(path)
        .expect("Couln't read CSV directory")
        .map(|f| f.unwrap().path().to_str().unwrap().to_owned())
        .collect()
}

/// Reads one CSV file into a Polars LazyFrame.
pub fn read_single_csv(filepath: &str) -> Result<LazyFrame, PolarsError> {
    Ok(LazyCsvReader::new(filepath)
        .has_header(true)
        .finish()
        .unwrap())
}

/// Takes a Vec of individual dataframes and returns a stacked dataframe,
/// with columns selected from the colnames argument.
pub fn stack_dfs(dfs: Vec<LazyFrame>, colnames: &Vec<String>) -> Result<DataFrame, PolarsError> {
    let big_df = concat(dfs, false, true);
    big_df.unwrap().collect().unwrap().select(colnames)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn stack_two_dfs() {
        let df1 = df! {
            "column_a" => &[1, 2, 3],
            "column_b" => &["a", "b", "c"]
        }
        .unwrap()
        .lazy();

        let df2 = df! {
            "column_a" => &[4, 5],
            "column_b" => &["d", "e"]
        }
        .unwrap()
        .lazy();

        let df3 = df! {
            "column_a" => &[1, 2, 3, 4, 5],
            "column_b" => &["a", "b", "c", "d", "e"]
        }
        .unwrap();
        let dfs = vec![df1, df2];
        let colnames = vec![String::from("column_a"), String::from("column_b")];
        let stacked = stack_dfs(dfs, &colnames).unwrap();
        assert_eq!(stacked, df3);
    }
}
