use std::io::{self, BufRead};

fn get_lookup_table() -> [(&'static str, usize); 20] {
    [
        ("zero", 0),
        ("0", 0),
        ("one", 1),
        ("1", 1),
        ("two", 2),
        ("2", 2),
        ("three", 3),
        ("3", 3),
        ("four", 4),
        ("4", 4),
        ("five", 5),
        ("5", 5),
        ("six", 6),
        ("6", 6),
        ("seven", 7),
        ("7", 7),
        ("eight", 8),
        ("8", 8),
        ("nine", 9),
        ("9", 9),
    ]
}

fn find_first(buf: &String) -> Option<usize> {
    let mut mem_index: Option<usize> = None;
    let mut mem_value: Option<usize> = None;
    for (pattern, value) in get_lookup_table() {
        match buf.find(pattern) {
            Some(index) => {
                if mem_index == None || Some(index) < mem_index {
                    mem_index = Some(index);
                    mem_value = Some(value);
                }
            },
            None => {},
        }
    }
    return mem_value;
} 

fn find_last(buf: &String) -> Option<usize> {
    let mut mem_index: Option<usize> = None;
    let mut mem_value: Option<usize> = None;
    for (pattern, value) in get_lookup_table() {
        match buf.rfind(pattern) {
            Some(index) => {
                if mem_index == None || mem_index < Some(index) {
                    mem_index = Some(index);
                    mem_value = Some(value);
                }
            },
            None => {},
        }
    }
    return mem_value;
} 

fn main() {
    let stdin = io::stdin();
    let mut total: usize = 0;
    for line in stdin.lock().lines() {
        let line = line.unwrap();
        let first = find_first(&line);
        let last = find_last(&line);
        match (first, last) {
            (Some(f), Some(l)) => total += f*10+l,
            (_, _) => {}
        }
    }
    println!("{}", total);
}
