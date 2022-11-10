use csv_stacker::{get_colnames, get_csv_filenames, stack_csvs};
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let csv_dir_path = &args[1];
    let output_path = &args[2];
    let filenames = get_csv_filenames(&csv_dir_path);
    let colnames = get_colnames("colnames.txt");
    stack_csvs(&filenames, &colnames, &output_path);
}
