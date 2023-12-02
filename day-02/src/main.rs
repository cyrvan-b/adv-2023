use std::io::{self, BufRead};
use std::cmp;

#[derive(Debug)]
struct Set {
    red: usize,
    green: usize,
    blue: usize,
}

impl Set {
    fn new() -> Self {
        Set {
            red: 0,
            green: 0,
            blue: 0,
        }
    }
}

fn get_all_sets(buf: &String) -> Vec<Set> {
    enum State {
        Ignore,
        ReadingSets,
    }
    let mut state: State = State::Ignore;
    let mut res: Vec<Set> = vec![];
    let mut cur_set: Set = Set::new();
    let mut cur_val: usize = 0;
    let mut cur_key: String = "".to_string();
    for character in buf.chars() {
        match state {
            State::Ignore => {
                match character {
                    ':' => state = State::ReadingSets,
                    _ => continue
                }
            },
            State::ReadingSets => {
                match character {
                    c if c.is_digit(10) => {
                        cur_val = cur_val * 10 + c.to_digit(10).expect("should not fail") as usize;
                    },
                    c if c.is_alphabetic() => {
                        cur_key.push(c);
                    },
                    ';' => {
                        match cur_key.as_str() {
                            "blue" => cur_set.blue = cur_val,
                            "green" => cur_set.green = cur_val,
                            "red" => cur_set.red = cur_val,
                            _ => {}
                        }
                        cur_val = 0;
                        cur_key.clear();
                        res.push(cur_set);
                        cur_set = Set::new();
                    },
                    ',' => {
                        match cur_key.as_str() {
                            "blue" => cur_set.blue = cur_val,
                            "green" => cur_set.green = cur_val,
                            "red" => cur_set.red = cur_val,
                            _ => {}
                        }
                        cur_val = 0;
                        cur_key.clear();
                    }
                    _ => {}
                }
            }
        }
    } 
    match cur_key.as_str() {
        "blue" => cur_set.blue = cur_val,
        "green" => cur_set.green = cur_val,
        "red" => cur_set.red = cur_val,
        _ => {}
    }
    cur_key.clear();
    res.push(cur_set);
    return res;
}

fn main() {
    let mut total: usize = 0;
    for line in io::stdin().lock().lines() {
       let line = line.unwrap();
       let mut cur: Set = Set::new();
       for set in get_all_sets(&line) {
           cur.green = cmp::max(cur.green, set.green);
           cur.blue = cmp::max(cur.blue, set.blue);
           cur.red = cmp::max(cur.red, set.red);
       }
       total += cur.green*cur.blue*cur.red;
    }
    println!("{total}");
}
