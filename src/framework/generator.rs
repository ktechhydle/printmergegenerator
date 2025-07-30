use csv::Writer;

pub fn generate_numbers(
    prefix: &str,
    column_count: usize,
    start: usize,
    end: usize,
    count_numbers_vertically: bool,
    aligned: bool,
) -> Result<String, String> {
    let num_range: Vec<usize> = (start..=end).collect();
    let max_length = num_range
        .last()
        .map(|&num| num.to_string().len())
        .unwrap_or(0);
    let mut output_buffer = Vec::new();
    let mut writer = Writer::from_writer(&mut output_buffer);
    let headers: Vec<String> = (0..column_count)
        .map(|i| format!("{}{}", prefix, i + 1))
        .collect();

    match writer.write_record(&headers) {
        Ok(_) => {}
        Err(e) => return Err(e.to_string()),
    }

    if count_numbers_vertically {
        let total_numbers = num_range.len();
        let num_data_rows = (total_numbers + column_count - 1) / column_count;
        let mut data_matrix: Vec<Vec<String>> =
            vec![vec!["".to_string(); column_count]; num_data_rows];

        for (i, &number) in num_range.iter().enumerate() {
            let formatted_number = if aligned {
                format!("{:0width$}", number, width = max_length)
            } else {
                number.to_string()
            };

            let col_idx = i / num_data_rows;
            let row_idx = i % num_data_rows;

            if row_idx < num_data_rows && col_idx < column_count {
                data_matrix[row_idx][col_idx] = formatted_number;
            }
        }

        for row in data_matrix {
            match writer.write_record(&row) {
                Ok(_) => {}
                Err(e) => return Err(e.to_string()),
            }
        }
    } else {
        let mut current_row: Vec<String> = Vec::new();

        for &number in num_range.iter() {
            let formatted_number = if aligned {
                format!("{:0width$}", number, width = max_length)
            } else {
                number.to_string()
            };
            current_row.push(formatted_number);

            if current_row.len() == column_count {
                match writer.write_record(&current_row) {
                    Ok(_) => {}
                    Err(e) => return Err(e.to_string()),
                }

                current_row = Vec::new();
            }
        }

        if !current_row.is_empty() {
            while current_row.len() < column_count {
                current_row.push("".to_string());
            }

            match writer.write_record(&current_row) {
                Ok(_) => {}
                Err(e) => return Err(e.to_string()),
            }
        }
    }

    let _ = writer.flush();
    drop(writer);

    Ok(match String::from_utf8(output_buffer) {
        Ok(string) => string,
        Err(e) => return Err(e.to_string()),
    })
}
