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

    fn is_in_bound(&self, x: usize, y: usize) -> bool {
        if y < self.rows.len() {
            if x < self.rows[y].len() {
                return true;
            }
        }
        return false;
    }

    fn is_number_at(&self, x: usize, y: usize) -> bool {
        if self.is_in_bound(x, y) {
            return self.rows[y].chars().nth(x).unwrap().is_digit(10);
        }
        return false;
    }
    
    fn get_number_at(&self, x: usize, y: usize) -> usize {
        let mut mx: usize = x;
        let my: usize = y;

        while 0 < mx && self.is_number_at(mx-1, my) {
            mx -= 1;
        }
        let mut n: usize = 0;
        while mx < self.rows[my].len() {
            let c = self.rows[my].chars().nth(mx).unwrap();
            if c.is_digit(10) {
                n = n * 10 + c.to_digit(10).unwrap() as usize;
            } else {
                break;    
            }
            mx += 1;
        }
        return n;
    }

    fn get_numbers_around(&self, x: usize, y: usize) -> Option<Vec<usize>> {
         if self.rows.len() == 0 {
             return None;
         }

         let x: i64 = x.try_into().unwrap();
         let y: i64 = y.try_into().unwrap();
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
         // 4123..  12.666
         // ..*...  ..*...
         // ...743  ......
         let mut res: Vec<usize> = vec![];
         for (ox, oy) in lookup_table {
            let tx = x+ox;
            let ty = y+oy;
            if 0 <= ty && ty < self.rows.len() as i64 {
                let ty = ty as usize;
                if 0 <= tx && tx < self.rows[ty].len() as i64 {
                    let tx = tx as usize;
                    if (ox == 0 || ox == 1) && (oy == -1 || oy == 1) {
                        if !self.is_number_at(tx-1, ty) && self.is_number_at(tx, ty) {
                            res.push(self.get_number_at(tx, ty));
                        }
                    }
                    else {
                        if self.is_number_at(tx, ty) {
                            res.push(self.get_number_at(tx, ty));
                        }
                    }
                }
            }
         }
         return Some(res);
    }
}

fn main() {
    let mut map = Map::new();
    for line in stdin().lock().lines() {
        let line = line.unwrap();
        map.add_cells(line);
    }

    let mut tot_num: usize = 0;
    for (y, row) in map.rows.iter().enumerate() {
        for (x, cell) in row.chars().enumerate() {
            match cell {
                '*' => {
                    match map.get_numbers_around(x, y) {
                        Some(n) if n.len() == 2 => tot_num += n[0] * n[1],
                        _ => {}
                    }
                },
                _ => {}
            }         
        } 
    }
    println!("{tot_num}");
}
