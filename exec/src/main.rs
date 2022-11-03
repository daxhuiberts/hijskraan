extern "C" {
    fn magic() -> i32;
    fn double(_: i32) -> i32;
}

fn main() {
    println!("magic value is {}", unsafe { magic() });
    println!("2 doubled is {}", unsafe { double(2) });
    println!("4 doubled is {}", unsafe { double(4) });
}
