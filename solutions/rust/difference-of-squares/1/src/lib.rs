pub fn square_of_sum(n: u32) -> u32 {
    let sum: u32 = (1..n+1).reduce(|accum, u| accum + u).unwrap_or(0);
    sum.pow(2)
}

pub fn sum_of_squares(n: u32) -> u32 {
    (1..n+1).reduce(|accum, u| accum + u.pow(2)).unwrap_or(0)
}

pub fn difference(n: u32) -> u32 {
    square_of_sum(n) - sum_of_squares(n)
}
