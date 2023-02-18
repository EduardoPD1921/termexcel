use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let file = match File::open("excel.csv") {
        Ok(f) => f,
        Err(_) => panic!("Erro ao abrir o arquivo.")
    };

    let lines = BufReader::new(file).lines();
    let mut raw_file_vec: Vec<String> = Vec::new();

    for line in lines {
        let l = line.unwrap();
        raw_file_vec.push(l);
    }

    let numerated_file_vec = insert_line_numeration(raw_file_vec);

    let biggests_cells_len_vec = get_vec_with_biggests_cells(&numerated_file_vec);
    let formatted_file_vec = convert_file_into_vec_with_usize(&numerated_file_vec, biggests_cells_len_vec);

    for cell in formatted_file_vec {
        println!("{}", cell);
    }
}

fn insert_line_numeration(raw_file_vec: Vec<String>) -> Vec<String> {
    raw_file_vec.iter().enumerate().map(|(index, item)| {
        format!("{}- {}", index + 1, item)
    }).collect::<Vec<String>>()
}

fn convert_file_into_vec_with_usize(raw_file_vec: &Vec<String>, biggests_cells_len: Vec<usize>) -> Vec<String> {
    let mut formatted_cells_vec: Vec<String> = Vec::new();

    for l in raw_file_vec {
        let splitted_cells: Vec<&str> = l.split(';').collect();
        let mut formatted_cell_vec: Vec<String> = Vec::new();

        for (index, cell) in splitted_cells.iter().enumerate() {
            let filled_cell = fill_cell(cell, biggests_cells_len[index]);
            formatted_cell_vec.push(format!("{}|", filled_cell));
        }

        let formatted_cell_slice: Vec<&str> = formatted_cell_vec.iter().map(|str| str.as_str()).collect();
        let formatted_cell_string = formatted_cell_slice.concat().to_owned();
        formatted_cells_vec.push(formatted_cell_string);
    }
    formatted_cells_vec
}

fn fill_cell(cell: &str, cell_target_size: usize) -> String {
    let mut filled_cell = cell.to_owned();
    while filled_cell.len() < cell_target_size {
        filled_cell.push(' ');
    }
    filled_cell
}

fn get_vec_with_biggests_cells(raw_file_vec: &Vec<String>) -> Vec<usize> {
    let mut biggests_cells_len: Vec<usize> = Vec::new();

    for l in raw_file_vec {
        let splitted_cells: Vec<&str> = l.split(';').collect();

        for (index, l) in splitted_cells.iter().enumerate() {
            set_biggest_cell(&mut biggests_cells_len, l, index)
        }
    }
    biggests_cells_len
}

fn set_biggest_cell(current_biggests_cells: &mut Vec<usize>, cell: &str, index: usize) {
    if current_biggests_cells.get(index).is_some() {
        if cell.len() > current_biggests_cells[index] {
            current_biggests_cells[index] = cell.len();
        }
    } else {
        current_biggests_cells.push(cell.len());
    }
}
