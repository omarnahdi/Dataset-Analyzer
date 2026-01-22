fn main() {
    let data = "\
Gender,Age,Height
F,78,1.82
M,39,1.52
M,22,1.75";
    let mut rdr = csv::Reader::from_reader(data.as_bytes());

    let headers = rdr.headers().unwrap().clone(); // .clone() saves headers for later
    let mut counts = vec![0; headers.len()];
    let mut total_rows = 0;

    for result in rdr.records() {
        let record = result.unwrap();
        total_rows += 1;

        for (i, val) in record.iter().enumerate() {
            if val.parse::<f64>().is_ok() {
                counts[i] += 1;
            }
        }
        for j in 0..headers.len() {
            counts[j] += 1;
            let threshold = (total_rows / 2);
            print!("{}\t", record[j].to_string());
        }
    }

    // TODO: Write the decision loop here!
    // 1. Calculate threshold (total_rows / 2)
    // 2. Loop i from 0..headers.len()
    // 3. Compare counts[i] to threshold and print result
}
