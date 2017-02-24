fn main() {
    let (x, y) = (32, 12);

    println!("(x,y)==({},{})",x,y);

    let (x, y) = (y, x);

    println!("(x,y)==({},{})",x,y);
    
}
