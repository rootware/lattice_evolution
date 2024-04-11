extern crate csv;
extern crate ndarray;
extern crate ndarray_csv;

use csv::ReaderBuilder;
use ndarray::{Array2, Axis};
use ndarray_csv::{Array2Reader, ReadError};

pub fn read_csv_file(filename: &str) -> Result<Array2<f64>, ReadError> {
    let mut reader = ReaderBuilder::new()
        .delimiter(b'\t')
        .from_path(filename).unwrap();

    let array: Array2<f64> = reader.deserialize_array2_dynamic().unwrap();
    
    Ok(array)
}

pub fn load_data() -> Array2<f64>{
    let filename = "./SP_Bayesianpriors_FinerRun/test_copy.tsv";
    match read_csv_file(filename) {
        Ok(array) => {
            return array;
            // Now you have your array loaded into an ndarray
        },
        Err(err) => eprintln!("Error reading CSV file: {}", err),
    }
    return Array2::<f64>::zeros((2,3));
}
