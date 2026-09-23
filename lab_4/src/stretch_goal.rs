use std::collections::HashMap;
use std::fs;

pub fn run() {
    let filename = "students.csv";

    let contents = match fs::read_to_string(filename) {
        Ok(data) => data,
        Err(_) => {
            println!("Could not read {}", filename);
            return;
        }
    };

    let mut lines = contents.lines();

    // Read the header row
    let headers: Vec<String> = lines
        .next()
        .unwrap()
        .split(',')
        .map(|s| s.trim().to_string())
        .collect();

    // Store rows as Vec<HashMap<String, String>>
    let mut rows: Vec<HashMap<String, String>> = Vec::new();

    for line in lines {
        let values: Vec<&str> = line.split(',').collect();

        let mut row = HashMap::new();

        for (header, value) in headers.iter().zip(values.iter()) {
            row.insert(header.clone(), value.trim().to_string());
        }

        rows.push(row);
    }

    println!("===== All Students =====");

    for row in &rows {
        println!("{:?}", row);
    }

    println!("\n===== Students with Age = 20 =====");

    for row in &rows {
        if row.get("Age").unwrap() == "20" {
            println!("{:?}", row);
        }
    }

    let mut total = 0.0;
    let mut count = 0;

    for row in &rows {
        if let Some(score) = row.get("Score")
            && let Ok(value) = score.parse::<f64>()
        {
            total += value;
            count += 1;
        }
    }

    if count > 0 {
        println!("\nAverage Score = {:.2}", total / count as f64);
    }
}
