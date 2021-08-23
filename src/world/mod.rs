use crate::cell;
use crossterm::event::KeyCode;
use crossterm::style::Stylize;
use std::fmt::Display;
use std::io::{BufWriter, Stdout, Write};

pub struct World {
    cells: Vec<Vec<cell::Cell>>,
    rows: usize,
    columns: usize,
    pub world_pointer: (usize, usize),
    pub calculus_speed: u64,

    // true -> update / false -> edit
    pub world_static: bool,

    //编辑模式指针位置
    pub print_point: (usize, usize),

    //窗口打印地图面积
    print_size: (usize, usize),
}

impl Display for World {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for i in self.print_point.0..self.print_point.0 + self.print_size.0 {
            for j in self.print_point.1..self.print_point.1 + self.print_size.1 {
                let cell = &self.cells[i][j];
                let (x, y) = self.world_pointer;
                let s;
                if self.world_static == false && x == i && y == j {
                    s = "<>".red();
                } else {
                    s = "  ".red();
                }
                if cell.is_alive() == true {
                    write!(f, "{}", s.on_white()).unwrap();
                } else {
                    write!(f, "{}", s).unwrap();
                }
            }
            write!(f, "\n").unwrap();
        }
        match self.world_static {
            true => {
                write!(
                    f,
                    "{}    {}",
                    "  calculus  ".bold().on(crossterm::style::Color::Rgb {
                        r: 130,
                        g: 57,
                        b: 53,
                    }),
                    "    edit    ".red(),
                )
                .unwrap();
            }
            false => {
                write!(
                    f,
                    "{}    {}",
                    "  calculus  ".red(),
                    "    edit    ".bold().on(crossterm::style::Color::Rgb {
                        r: 130,
                        g: 57,
                        b: 53,
                    }),
                )
                .unwrap();
            }
        }
        write!(
            f,
            "    {}{}{}   ",
            "calculus speed: ".red(),
            self.calculus_speed.to_string().red(),
            "ms".red()
        )
        .unwrap();
        Ok(())
    }
}

impl World {
    pub fn new(rows: usize, columns: usize, x: usize, y: usize) -> World {
        let r: Vec<cell::Cell> = vec![cell::Cell::new(false); columns];
        let world: Vec<Vec<cell::Cell>> = vec![r; rows];
        World {
            cells: world,
            rows,
            columns,
            world_pointer: (0, 0),
            calculus_speed: 10,
            world_static: true,
            print_point: (3, 3),
            print_size: (x, y),
        }
    }

    pub fn update_world(&mut self, handle: &mut BufWriter<Stdout>) {
        let mut cells = self.cells.clone();
        for i in 0..self.rows {
            for j in 0..self.columns {
                let alive_number = self.alive_friends(i, j);
                if self.cells[i][j].is_alive() {
                    if alive_number < 2 || alive_number > 3 {
                        cells[i][j].change_state();
                    }
                } else {
                    if alive_number == 3 {
                        cells[i][j].change_state();
                    }
                }
            }
        }

        self.cells = cells;
        self.print(handle);
    }

    // i=true时 方向键作用为移动打印地图位置 反之为移动指针
    pub fn edit_world(&mut self, key_code: KeyCode, handle: &mut BufWriter<Stdout>, i: bool) {
        match key_code {
            KeyCode::Down => {
                self.move_pointer(1, 0, i);
            }
            KeyCode::Up => {
                self.move_pointer(-1, 0, i);
            }
            KeyCode::Left => {
                self.move_pointer(0, -1, i);
            }
            KeyCode::Right => {
                self.move_pointer(0, 1, i);
            }
            KeyCode::Char(' ') => {
                if i == true {
                    let (x, y) = self.world_pointer;
                    self.cells[x][y].change_state();
                }
            }
            _ => {}
        }
        self.print(handle);
    }

    pub fn set_cell_state(&mut self, i: usize, j: usize) {
        self.cells[i][j].change_state();
    }

    pub fn print(&self, handle: &mut BufWriter<Stdout>) {
        print!("\x1b[H");
        write!(handle, "{}", self).unwrap();
        handle.flush().unwrap();
    }

    fn alive_friends(&self, i: usize, j: usize) -> usize {
        let x = [1, -1, 0, 0, 1, -1, -1, 1];
        let y = [0, 0, 1, -1, 1, -1, 1, -1];
        let mut alive_number: usize = 0;
        for p in 0..8 {
            let p1 = i as i32 + x[p];
            let p2 = j as i32 + y[p];
            if p1 >= 0 && p1 < self.rows as i32 && p2 >= 0 && p2 < self.columns as i32 {
                let p1 = p1 as usize;
                let p2 = p2 as usize;
                if self.cells[p1][p2].is_alive() {
                    alive_number += 1;
                }
            }
        }
        alive_number
    }

    fn move_pointer(&mut self, x: i32, y: i32, update_mod: bool) {
        if update_mod == true {
            let (i, j) = self.world_pointer;
            let i = i as i32 + x;
            let j = j as i32 + y;
            if i >= self.print_point.0 as i32
                && j >= self.print_point.0 as i32
                && (self.print_point.0 + self.print_size.0) as i32 > i
                && (self.print_point.1 + self.print_size.1) as i32 > j
            {
                self.world_pointer.0 = i as usize;
                self.world_pointer.1 = j as usize;
            }
        } else {
            let (i, j) = self.print_point;
            let i = i as i32 + x;
            let j = j as i32 + y;
            if i >= 0
                && j >= 0
                && self.rows as i32 > i + self.print_size.0 as i32
                && self.columns as i32 > j + self.print_size.1 as i32
            {
                self.print_point.0 = i as usize;
                self.print_point.1 = j as usize;
            }
        }
    }
}
