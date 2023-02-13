use gameoflife::*;
use std::{thread, time};

fn main() {
    let gun: Vec<(usize, usize)> = vec![
    (13, 2),  (14, 2),  (13, 3),  (14, 3),  (13, 12), (14, 12), (15, 12), (12, 37),
    (12, 13), (16, 13), (11, 14), (17, 14), (11, 15), (17, 15), (14, 16), (12, 17),
    (16, 17), (13, 18), (14, 18), (15, 18), (14, 19), (11, 22), (12, 22), (13, 22), 
    (11, 23), (12, 23), (13, 23), (10, 24), (14, 24), (9, 26),  (10, 26), (14, 26), 
    (15, 26), (11, 36), (12, 36), (11, 37)];

    game_loop(Board::from(gun));
}

fn game_loop(mut board: Board){
    loop{
        let start = time::Instant::now();
        print!("\x1B[2J\x1B[1;1H");
        board.draw();
        board.update();
        println!("Frame time: {:?}", start.elapsed());
        thread::sleep(time::Duration::from_millis(100));
    }
}



