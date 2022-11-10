use csv_stacker::read_single_csv;

fn main() {
    let df = read_single_csv("csvs/foo.csv").unwrap();
    println!("{}", df);
}
