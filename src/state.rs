use crate::curses::*;
use crate::filter::Filter;
use libm::tanh;
pub type State = Vec<Vec<f64>>;
pub fn random_state(rows : usize, cols : usize) -> State {
    let mut state = vec![ vec![0f64;cols ] ; rows ];
    for row in 0..rows {
        for col in 0..cols {
            state[row][col] = 1.0 - 2.0*rand::random::<f64>();
        }
    }
    state
}
pub fn display(state : &State, rows : usize, cols : usize, right : usize, down : usize) -> () {
    for row in 0..rows {
        cursor_to(row + down + 2, right);
        for col in 0..cols {
            let brightness = hue(state[row][col]);
            set_color(brightness, brightness,brightness);
            print!("\u{2588}");
        }
    }  
}

pub fn hue(x : f64) -> u8 {
    (128.0 * (x + 1.0)).trunc() as u8

}
pub fn filter_state_mutate_cell(filter : &Filter, state : &mut State, center_row : usize, center_col : usize,rows : usize, cols : usize) {
    let mut sum = 0f64;
    for touch in filter {
        let row = cyclical_index(touch.row + center_row as i32, rows as i32);
        let col = cyclical_index(touch.col + center_col as i32, cols as i32);
        sum += touch.value*state[row as usize][col as usize];
    }
    state[center_row][center_col] = tanh(sum);
}
pub fn cyclical_index(i : i32, limit : i32) -> i32 {
    if i < 0 { return i + limit; }
    if i >= limit { return i - limit };
    i
}