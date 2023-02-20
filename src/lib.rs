use core::ops::Range;
use std::{thread, time::{self, Duration}, collections::HashMap};

const GRIDLEN: usize = 40;
const MAX_INDEX: usize = GRIDLEN-1;

// Trait Definitions //
pub trait Draw {
    fn draw(&self);
}

pub trait Configure {
    fn configure(&mut self, c: Config);
}


// Config 
pub struct Config{
    pub args: Vec<String>,
    pub draws: bool,
    pub returns: bool,
    pub sleeptime: u64
}


// Results
pub struct Results{
    final_board: Board,
    final_count: u64,
    repeating_key: u64,
}

impl Results{
    pub fn to_board(&self) -> Board{
        Board::from(self.final_board.get_state())
    }
}

impl Draw for Results{
    fn draw(&self){
        // index == self.repeating_key, count == self.final_count

        self.final_board.draw();
        let pattern_length = self.final_count - self.repeating_key;
        
        match pattern_length{ 
            0 => println!("Pattern became static at iteration: {}", self.repeating_key),

            1 => println!("Alternating pattern at iteration: {}", self.repeating_key),

            _ => println!("Repeating pattern between iterations {} and {}
                          \nLooping pattern length: {}",
                          self.repeating_key, self.final_count, pattern_length),
        }
    }
}


// Game
pub struct Game{
    board: Board,
    past_states: HashMap<Vec<Vec<bool>>, u64>,
    sleeptime: time::Duration,
    draw: bool,
    returns: bool
}

impl Game{
    pub fn run(&mut self) -> Option<Results>{
        let mut count: u64 = 0;        
        loop{
            count += 1;
            self.past_states.insert(self.board.get_state(), count);
            self.tick();
            let new_state = self.board.get_state();
            if self.past_states.contains_key(&new_state){
                match self.returns{
                    true => return Some(
                        Results { 
                            final_board: (self.board.clone()),
                            final_count: (count),
                            repeating_key: *self.past_states.get(&new_state).unwrap()
                        }),

                    false => {
                        return None;
                    }
                }
            }
        }
    }


    fn set_sleeptime(&mut self, n: &u64){
        self.sleeptime = Duration::from_millis(*n);
    }

    fn set_draw(&mut self, b: &bool){
        self.draw = *b;
    }

    fn set_returns(&mut self, b: &bool){
        self.returns = *b;
    }

    fn tick(&mut self){
        if self.draw{
            self.board.draw();
            thread::sleep(self.sleeptime);
        }
        self.board.update();
    }
}

impl From<Board> for Game{
    fn from(board: Board) -> Self{
        Game { 
            board: board,    
            past_states: HashMap::new(),
            sleeptime: time::Duration::from_millis(25),
            draw: true,
            returns: true,
        }
    }
}

impl From <Vec<(usize, usize)>> for Game {
    fn from(coordinates: Vec<(usize, usize)>) -> Self{
        Game { 
            board: Board::from(coordinates),    
            past_states: HashMap::new(),
            sleeptime: time::Duration::from_millis(25),
            draw: true,
            returns: true,
        }
    }
}

impl Configure for Game{
    fn configure(&mut self, config: Config) {
        self.set_draw(&config.draws);
        self.set_sleeptime(&config.sleeptime);
        self.set_returns(&config.returns);
    }
}


// Board
#[derive(Hash, Eq, PartialEq)]
pub struct Board{
    grid: Vec<Vec<bool>>,
    swaps: Vec<(usize, usize)>,
}

impl Board{
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

    pub fn get_state(&self) -> Vec<Vec<bool>>{
        self.grid.clone()
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

impl From<Vec<(usize, usize)>> for Board{
    fn from(coordinates: Vec<(usize, usize)>) -> Self{

        let mut starting_grid = vec![vec![false; GRIDLEN]; GRIDLEN];

        for i in coordinates{
            if i.0 < GRIDLEN && i.1 < GRIDLEN{
                starting_grid[i.0][i.1] = true;
            }
        }

        
        Board{grid: starting_grid, swaps: Vec::new()}
    }
}

impl Clone for Board{
    fn clone(&self) -> Self {
        Board { grid: self.grid.clone(), swaps: self.swaps.clone() }
    }
}

impl From<Vec<Vec<bool>>> for Board{
    fn from(grid: Vec<Vec<bool>>) -> Self{
        Board { grid: grid, swaps: Vec::new() }
    }
}

impl Draw for Board{
    fn draw(&self){
        let mut grid = String::new();
        let floor = String::from_iter(["# "; GRIDLEN+2]);
        grid.push_str(&(floor.clone() + "\n"));

        for i in &self.grid{
            let line = String::from_iter(i.iter().map(|a| match a{
                false => "  ",
                true => "# "
            }));
            grid.push_str("# ");
            grid.push_str(&(line + "#\n"));
        }
        grid.push_str(&(floor));
        println!("{esc}[2J{esc}[1;1H{grid}", esc = 27 as char);
    }
}
