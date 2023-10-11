#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let scale = 2;
    let rect1 = Rectangle {
        width: dbg!(30 * scale),
        height: 50,
    };

    print_rect(&rect1);
    dbg!(&rect1);
}

fn print_rect(r: &Rectangle) {
    println!("w:{} h:{}", r.width, r.height)
}