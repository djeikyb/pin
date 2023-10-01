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
    let length = match length_unparsed.parse::<u32>() {
        Ok(n) => n,
        Err(_) => {
            eprintln!("Not a number: {length_unparsed}");
            std::process::exit(-1);
        }
    };

    let mut rng = thread_rng();
    let max = match 10u32.checked_pow(length) {
        None => {
            eprintln!("Can't generate that long of a pin.");
            std::process::exit(-1);
        }
        Some(n) => n
    };
    let pin = rng.gen_range(0..max);

    println!("{}", pin);
}