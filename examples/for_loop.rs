use veclite::Vel;

fn main() {
    let mut numbers = Vel::new();
    numbers.add(1);
    numbers.add(2);
    numbers.add(3);

    // Using a for loop to iterate by value (consuming the Veclite)
    for n in numbers.clone() {
        println!("By value: {}", n);
    }

    // Using a for loop to iterate by reference (does not consume the Veclite)
    for n in &numbers {
        println!("By reference: {}", n);
    }

    // Using a for loop to iterate by mutable reference (allows mutation)
    for n in &mut numbers.clone() {
        println!("By mutable reference: {}", n);
    }
}