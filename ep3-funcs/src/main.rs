fn main() {
    println!("Hello, world!");

    another_function(5, 'a');

    let x = not_five();
    println!("The value of x is: {}", x);
}

fn another_function(value: i32, unit_label: char) {
    println!("Another function doing: {value} {unit_label}");
}

fn not_five() -> i32 {
    10
}
