use veclite::Veclite;

fn main() {
    let mut list = Veclite::new();
    list.add(10);
    list.add(20);
    list.prepend(5);
    println!("{}", list); // Output: 5 10 20
}