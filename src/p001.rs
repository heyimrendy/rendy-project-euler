// https://projecteuler.net/problem=1

// Change final_number to u32 because sum result might return u32(problem max number 1000)
// but cannot accept u16
#[allow(dead_code)]
fn solve(final_number: u32) -> u32 {
    (1..final_number)
        .filter(|&x| x % 3 == 0 || x % 5 == 0)
        .sum()
}

#[allow(dead_code)]
pub fn run() {
    println!(
        "Sum of all the multiples of 3 or 5 below 1000 is {}",
        solve(1000)
    )
}
