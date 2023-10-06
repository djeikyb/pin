use rand::{
    Rng,
    thread_rng,
};

fn main() {
    let length_unparsed = match std::env::args().nth(1) {
        None => {
            eprintln!("Create a 5 digit pin like: `pin 5`");
            std::process::exit(-1);
        }
        Some(a) => a
    };

    let length = match length_unparsed.parse::<usize>() {
        Ok(n) => n,
        Err(_) => {
            eprintln!("Not a number or too big: {length_unparsed}");
            std::process::exit(-1);
        }
    };

    let mut rng = thread_rng();
    let mut arr: Vec<u8> = Vec::with_capacity(length);
    arr.resize(length, 0u8);
    for i in (0..length).rev() {
        arr[i] = rng.gen_range(0..10) + 48;
    }
    let pin = std::str::from_utf8(arr.as_slice()).unwrap();
    println!("{}", pin);
}