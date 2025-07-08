use veclite::Veclite;

fn main() {
    let mut names = Veclite::new();
    names.add("Alice".to_string());
    names.add("Bob".to_string());
    names.add("Carol".to_string());
    println!("{}", names); // Output: Alice Bob Carol

    // Remove the first element
    names.remove(0);
    println!("{}", names); // Output: Bob Carol
}