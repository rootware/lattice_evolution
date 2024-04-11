extern crate csv;
extern crate ndarray;
extern crate ndarray_csv;
use std::fs::File;
use csv::ReaderBuilder;
use ndarray::{Array2, Axis};
use ndarray_csv::{Array2Reader, ReadError};


pub fn read_csv_file(filename: &str) -> Result<Array2<String>, ReadError> {

    let file = File::open(filename).unwrap();
    let mut reader = ReaderBuilder::new()
        .has_headers(false)
        .delimiter(b'\t')
        .from_reader(file);

    let result = reader.deserialize_array2_dynamic();
    let array: Array2<String> = result.unwrap();
    
    Ok(array)
}

pub fn load_data() -> Array2<f64>{
    let filename = "./SP_Bayesianpriors_FinerRun/test_copy.tsv";
    match read_csv_file(filename) {
        Ok(array) => {

            let mut result = Array2::<f64>::zeros(array.dim());

            for ((i, j), &ref element) in array.indexed_iter() {
                if let Ok(parsed) = element.parse::<f64>() {
                    result[[i, j]] = parsed;
                } 
            }
            return result;
            // Now you have your array loaded into an ndarray
        },
        Err(err) => eprintln!("Error reading CSV file: {}", err),
    }
    return Array2::<f64>::from_elem((2,2), -1.0);
}
