use veclite::Veclite;

fn main() {
    let mut list = Veclite::new();
    list.push(10);
    list.push(20);
    list.prepend(5);
    println!("{}", list); // Output: 5 10 20
}