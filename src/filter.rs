pub struct Touch {
    pub row : i32,
    pub col : i32,
    pub value : f64,
}

pub type Filter = Vec<Touch>;
pub fn random_filter(height : i32, width : i32, power : f64) -> Filter {
    let mut filter = vec!();
    debug_assert!(height%2 == 1 && width%2 == 1);
    let row_span = height/2;
    let col_span = width/2;
    for row in -row_span..=row_span {
        for col in -col_span..=col_span {
            filter.push(Touch { row , col,  value : power * (1.0 -2.0*rand::random::<f64>() )});
        }
    }
    filter
}




