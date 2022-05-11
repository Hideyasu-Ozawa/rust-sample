use rand::Rng;

fn main() {
    println!("Hello, world!");

    let mut rng = rand::thread_rng();
    let i: i32 = rng.gen();
    println!("i = {}", i);
}
