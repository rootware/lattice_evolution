use std::fs::File;
use std::fs::OpenOptions;
use std::io::Write;
use std::io::{BufReader, BufRead};

pub fn read() -> (Vec<f64>, Vec<f64>) {
    let file =  File::open("./Catie/QOCshaking.txt").unwrap();


    let _file2 = File::create("./Catie/test_read.txt").unwrap();
    let mut file2 = OpenOptions::new()
    .write(true)
    .append(true)
    .open("./Catie/test_read.txt").unwrap();
    
    // Create a buffered reader to read lines from the file
    let reader = BufReader::new(file);
    
    // Vector to store the time
    let mut time: Vec<f64> = Vec::new();
    let mut shakingfunction : Vec<f64> = Vec::new();
    
    // Iterate over each line in the file
    for line in reader.lines() {
        // Parse each line into time
        if let Ok(line) = line {
    
            let mut x = line.split_whitespace();
            let time_val = x.next().unwrap();
            let shake_val = x.next().unwrap(); 
    
            if let Ok(num) = time_val.parse::<f64>() {
                time.push(num*1.0e-3);//values are written in ms, not s
            } else {
                eprintln!("Failed to parse float: {}", time_val);
            }
    
            if let Ok(num) = shake_val.parse::<f64>() {
                shakingfunction.push(num);
            } else {
                eprintln!("Failed to parse float: {}", shake_val);
            }
            
        } else {
            eprintln!("Error reading line");
        }
    }
    let mut s = String::new();
    for num in &shakingfunction {
        s.push_str(&num.to_string());
        s.push_str("\n");
        }

        file2
        .write_all( s.as_bytes())
        .unwrap();


    (time, shakingfunction)
}

