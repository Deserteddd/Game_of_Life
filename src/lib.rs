use core::ops::Range;
use std::{thread, time, collections::HashMap};

const GRIDLEN: usize = 40;
const MAX_INDEX: usize = GRIDLEN-1;

#[derive(Hash, Eq, PartialEq)]
pub struct Board{
    grid: [[bool; GRIDLEN]; GRIDLEN],
    swaps: Vec<(usize, usize)>
}

pub struct Game{
    board: Board,
    past_states: HashMap<[[bool; GRIDLEN]; GRIDLEN], u64>,
    sleeptime: time::Duration
}

impl Game{
    pub fn run(&mut self){
        let mut count: u64 = 1;
        loop{
            let start = time::Instant::now();
            self.past_states.insert(self.board.get_state(), count);
            print!("\x1B[2J\x1B[1;1H");
            self.tick();
            if self.past_states.contains_key(&self.board.get_state()){
                let index = self.past_states.get(&self.board.get_state()).unwrap();
                self.print_results(index, &count);
                break;
            } else {
                count += 1;
            }
            // println!("Frame time: {:?}", start.elapsed());
            // thread::sleep(time::Duration::from_millis(50));
        }
    }

    fn print_results(&self, index: &u64, count: &u64){
        let pattern_length = count-index;
        match pattern_length{
            1 => println!("Pattern became static at iteration: {index}"),
            _ => println!("repeating pattern between iterations {} and {}
                          \nLooping pattern length: {}",index, count, pattern_length),
        }


    }

    pub fn set_sleeptime(&mut self, n: u64){
        self.sleeptime = time::Duration::from_millis(n);
    }

    fn tick(&mut self){
        self.board.update();
        self.board.draw();
    }
}

impl From<Board> for Game{
    fn from(board: Board) -> Self{
        Game { 
            board: board,    
            past_states: HashMap::new(),
            sleeptime: time::Duration::from_millis(10),
        }
    }
}

impl From<Vec<(usize, usize)>> for Board{
    fn from(coordinates: Vec<(usize, usize)>) -> Self{
        let mut board = Board{grid: [[false; GRIDLEN]; GRIDLEN],
                              swaps: Vec::new()};
        for i in coordinates{
            if i.0 < GRIDLEN && i.1 < GRIDLEN{
                board.grid[i.0][i.1] = true;
            }
        }
        board
    }
}

impl Board{
    pub fn draw(&self){
        let mut grid = String::new();
        let floor = String::from_iter(["# "; GRIDLEN+2]);
        grid.push_str(&(floor.clone() + "\n"));

        for i in self.grid{
            let line = String::from_iter(i.map(|a| match a{
                false => "  ",
                true => "# "
            }));
            grid.push_str("# ");
            grid.push_str(&(line + "#\n"));
        }

        grid.push_str(&(floor));
        print!("{}\n", grid);
    }
 
    pub fn update(&mut self){
        self.swaps.clear();
        for (x, i) in self.grid.iter().enumerate(){
            for (y, j) in i.iter().enumerate(){
                if *j{
                    match self.live_neighbours(x, y){
                        2 | 3 => continue,
                        _=> self.swaps.push((x, y))
                    }
                } else if self.live_neighbours(x, y) == 3{
                        self.swaps.push((x, y))
                }

            } 
        }

        for i in self.swaps.iter(){
            self.grid[i.0][i.1] ^= true; 
        }
    }

    pub fn get_state(&self) -> [[bool; GRIDLEN]; GRIDLEN]{
        self.grid
    }

    fn live_neighbours(&self, x: usize, y: usize) -> u8{
        let mut ans = 0;
        let x_range: Range<usize>;
        let y_range: Range<usize>;

        match x{
            0 => x_range = 0..2,
            MAX_INDEX => x_range = x-1..x+1,
            _=> x_range = x-1..x+2
        }

        match y{
            0 => y_range = 0..2,
            MAX_INDEX => y_range = y-1..y+1,
            _=> y_range = y-1..y+2
        }

        for i in x_range{
            for j in y_range.clone(){
                if self.grid[i][j] && !(i == x && j == y){
                    ans += 1;
                }
            }
        }

        ans
    }
}




