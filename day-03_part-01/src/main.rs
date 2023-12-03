use std::io::stdin;
use std::io::BufRead;

struct Map {
    rows: Vec<String>,
}

impl Map {
    fn new() -> Self {
        Self {
            rows: vec![],
        }
    }

    fn add_cells(&mut self, rows: String) {
        self.rows.push(rows);
    }

    fn has_symbols_near(&self, x: i64, y: i64) -> bool {
        if self.rows.len() == 0 {
            return false;
        }

        let lookup_table = [
             (-1, -1),
             ( 0, -1),
             ( 1, -1),
             (-1,  0),
             ( 0,  0),
             ( 1,  0),
             (-1,  1),
             ( 0,  1),
             ( 1,  1),
        ];
        for (ox, oy) in lookup_table {
            let tx = x+ox;
            let ty = y+oy;
            if 0 < ty && ty < self.rows.len() as i64 && 0 < tx && tx < (self.rows[ty as usize].len() as usize).try_into().unwrap() {
                let c = self.rows[ty as usize].chars().nth(tx as usize).unwrap();
                if !c.is_digit(10) && c != '.' {
                    return true;
                }
            }
        }
        return false;
    }
}

enum State {
    InNumberInvalid,
    InNumberValid,
    OutNumber,
}

fn main() {
    let mut map = Map::new();
    for line in stdin().lock().lines() {
        let line = line.unwrap();
        map.add_cells(line);
    }

    let mut state = State::OutNumber;
    let mut cur_num: usize = 0;
    let mut tot_num: usize = 0;
    for (y, row) in map.rows.iter().enumerate() {
        for (x, cell) in row.chars().enumerate() {
            match cell {
                c if c.is_digit(10) => {
                    cur_num = cur_num * 10 + c.to_digit(10).unwrap() as usize;
                    match state {
                        State::InNumberValid => {},
                        State::InNumberInvalid => {
                            if map.has_symbols_near(x.try_into().unwrap(), y.try_into().unwrap()) {
                                state = State::InNumberValid;
                            }
                        },
                        State::OutNumber => {
                            state = State::InNumberInvalid;
                            if map.has_symbols_near(x.try_into().unwrap(), y.try_into().unwrap()) {
                                state = State::InNumberValid;
                            }
                        },
                    }
                },
                _ => {
                    match state {
                        State::InNumberValid => {
                            tot_num += cur_num; 
                        },
                        State::InNumberInvalid => {},
                        State::OutNumber => {},
                    }
                    state = State::OutNumber;
                    cur_num = 0;
                }
            }         
        } 
    }
    println!("{tot_num}");
}
