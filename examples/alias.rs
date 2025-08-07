use veclite::Vel;

fn main() {
    let mut numbers = Vel::new();
    for i in 1..=3 {
        numbers.push(i);
    }
    // Iterate over elements
    for n in numbers.iter() {
        print!("[{}]", n);
    }
    println!(); // Output: [1][2][3]
}