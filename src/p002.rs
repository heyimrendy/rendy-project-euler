// https://projecteuler.net/problem=2

#[allow(dead_code)]
pub fn run() {
    let mut sum_num: Vec<u32> = vec![2];
    let mut previous: u32 = 1;
    let mut current: u32 = 2;
    loop {
        let tmp_previous = current;
        current = previous + current;
        previous = tmp_previous;

        if current > 4000000 {
            break;
        }

        if current % 2 == 0 {
            sum_num.push(current)
        }
    }

    // Use iter() instead into_iter() because we only need to read the data.
    let arr_sum: u32 = sum_num.iter().sum();
    println!(
        "Sum of the even-valued terms of fibonnaci numbers below 4 million is {}",
        arr_sum
    )
}
