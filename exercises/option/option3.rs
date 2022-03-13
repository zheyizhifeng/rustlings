// option3.rs
// Make me compile! Execute `rustlings hint option3` for hints


#[derive(Debug)]
struct Point {
    x: i32,
    y: i32,
}
fn t<T>(_: &T) {
    println!("type is {}", std::any::type_name::<T>());
}
fn main() {
    let y: Option<Point> = Some(Point { x: 100, y: 200 });
    // t(&y);
    match &y {
        Some(p) => {
            t(p);
            println!("Co-ordinates are {},{} ", p.x, p.y);
        }
        _ => println!("no match"),
    }
    // print!("y is {:?}", y.as_ref().unwrap());
    // print!("point is {}", Point { x: 100, y: 200 });
    y; // Fix without deleting this line.
}
