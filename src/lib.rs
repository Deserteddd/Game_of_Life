use core::ops::Range;

const GRIDLEN: usize = 40;
const MAX_INDEX: usize = GRIDLEN-1;

pub struct Board{
    grid: [[bool; GRIDLEN]; GRIDLEN],
    swaps: Vec<(usize, usize)>
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




