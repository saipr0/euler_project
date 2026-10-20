fn main() {
    println!("Zero = {}", zero(363_000));
    println!("One = {}", one(1000));
}

fn zero(n: usize) -> usize {
    // given n square numbers, return sum of only odd sqaures
    let count: usize = n / 2; // count of odd numbers

    count*(4*count*count-1)/3
}

fn one(n: usize) -> usize {
    // find the sum of all multiples of 3 or 5 below 1000
    let p = (n - 1) / 3;
    let q = (n - 1) / 5;
    let r = (n - 1) / 15;
    return (3*p*(p+1) + 5*q*(q+1) - 15*r*(r+1)) / 2
}
