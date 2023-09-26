mod curses; use curses::*;
mod filter; use filter::*;
mod state; use state::*;
use rand::Rng;
use std::env;
use std::process::exit;  


fn main() {
    
    let args: Vec<String> = env::args().collect();
    if args.len() < 7 {
        print!("To use Tornado, add parameters in the following order :\n");
        print!("tornado [rows] [cols] [span] [flux] [updates per step] [steps]\n");
        reset();
        exit(0);

    }
    clear_screen();
    let mut default_settings: Vec<usize> =vec![25,50,7,5,20,3000,500];
    for i in 0..default_settings.len() {
        if i < args.len() - 1 {
            let parsed = match args[i+1].parse::<usize>() {
                Ok(u) => u,
                Err(_) => 0,
            };
            if parsed > 0 && i < default_settings.len() {
                default_settings[i] = parsed;
            }
        }
    }
    let rows = default_settings[0];
    let cols = default_settings[1];
    let filters = 1;
    let span = default_settings[2] as i32;
    let flux = default_settings[3];
    let updates = default_settings[4];
    let duration = default_settings[5];
    let right = 10;
    let down = 0;
    let mut rng = rand::thread_rng();
    let mut row = rng.gen_range(0..rows);
    let mut col = rng.gen_range(0..cols);
    clear_screen();
    hide_cursor();
    let mut filter = random_filter(span,span, 0.5 + 0.5*rand::random::<f64>());
    let mut state = random_state(rows, cols);

    for i in 0..duration {
        for _ in 0..updates {
            filter_state_mutate_cell(&filter, &mut state, row, col, rows, cols);
            let mut r =  row as i32;
            let mut c =  col as i32;
            match rand::random::<usize>()%4 {
                0 => {r = r + 1;},
                1 => {c = c + 1;},
                2 => {r = r - 1;},
                3 => {c = c - 1;},
                _ => {},
            };
            row  = cyclical_index(r , rows as i32) as usize;
            col = cyclical_index(c , cols as i32) as usize;
            if rand::random::<usize>()%100000 < flux {
                    filter = random_filter(span,span,0.4 + 0.4*rand::random::<f64>());
                }
            }
        display(&state, rows, cols, right, down+1);
        set_color(255,255,255);
        cursor_to(down,right);
        print!("tornado {} {} {} {} {} {}", rows, cols, span, flux, updates, duration-i);


        

    }
    reset();

}
