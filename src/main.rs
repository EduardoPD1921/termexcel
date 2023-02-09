use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let file = match File::open("excel.csv") {
        Ok(f) => f,
        Err(_) => panic!("Erro ao abrir o arquivo.")
    };
    let (file_as_vec, biggest_cell_len) = convert_file_into_vec_with_usize(file);
}

fn convert_file_into_vec_with_usize(file: File) -> (Vec<String>, usize) {
    let lines = BufReader::new(file).lines();
    let mut formatted_cells_as_vec: Vec<String> = Vec::new();
    let mut biggest_cell_len: usize = 0;

    for line in lines {
        let l = line.unwrap();
        let splitted_cells: Vec<&str> = l.split(';').collect();
        let mut formatted_cell_vec: Vec<String> = Vec::new();

        for l in splitted_cells {
            if l.len() != 0 {
                set_biggest_cell(&mut biggest_cell_len, l);
                formatted_cell_vec.push(format!("{}|", l));
            }
        }

        let formatted_cell_slice: Vec<&str> = formatted_cell_vec.iter().map(|str| str.as_str()).collect();
        let formatted_cell_string = formatted_cell_slice.concat().to_owned();
        formatted_cells_as_vec.push(formatted_cell_string);
    }
    (formatted_cells_as_vec, biggest_cell_len)
}

fn set_biggest_cell(current_big_cell: &mut usize, cell: &str) {
    if cell.len() > *current_big_cell {
        *current_big_cell = cell.len();
    }
}
