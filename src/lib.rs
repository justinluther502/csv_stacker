use polars::prelude::*;

mod config;

pub fn read_single_csv(filepath: &str) -> Result<DataFrame, PolarsError> {
    let colnames = config::lines_from_file("colnames.txt");
    let df = LazyCsvReader::new(filepath)
        .has_header(true)
        .finish()?
        .collect()
        .unwrap();
    df.select(colnames)
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
