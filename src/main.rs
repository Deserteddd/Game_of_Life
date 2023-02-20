use gameoflife::*;
use std::time::*;

fn main() {
    let p1: Vec<(usize, usize)> = vec![
        (13, 2),  (14, 2),  (13, 3),  (14, 3),  (13, 12), (14, 12), (15, 12), (12, 37),
        (12, 13), (16, 13), (11, 14), (17, 14), (11, 15), (17, 15), (14, 16), (12, 17),
        (16, 17), (13, 18), (16, 18), (15, 18), (14, 19), (11, 2), (12, 22), (13, 22), 
        (11, 23), (12, 23), (13, 23), (10, 24), (14, 24), (9, 26),  (10, 26), (14, 26), 
        (15, 26), (11, 36), (12, 36), (11, 37)];
    

    let p1_config = Config{
        args: Vec::new(),
        draws: false,
        sleeptime: 0
    };

    let now = Instant::now();
    run(p1, p1_config);

    println!("Runtime: {:?}", now.elapsed());
    //test
    
}


fn run(pattern: Vec<(usize, usize)>, config: Config){

        let mut game = Game::from(pattern);
        game.configure(config);
        game.run().draw();
}
