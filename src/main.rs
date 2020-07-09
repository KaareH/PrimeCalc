use std::io;
use std::{thread, time};
use std::collections::HashMap;

fn main() {
    println!("Hello, welcome to PrimeCalc!");

    let mut primes :HashMap<u32, u64> = HashMap::new();

    loop {
        println!("Please enter the index of which prime number you want to find!");
        let mut input = String::new();
        match io::stdin().read_line(&mut input) {
            Ok(_) => {
                if input.trim() == "exit" {
                    break;
                }

                match input.trim().parse::<u32>() {
                    Ok(index) => {
                        println!("You have entered: {}", index);
                        println!("{} is prime #{}", check_index(index, &mut primes), index);
                    },
                    Err(_) => {
                        println!("Please enter a natural number!");
                    }
                }
            },
            Err(e) => {
                println!("Error! {}", e);
            }
        }

        thread::sleep(time::Duration::from_secs(1));

    }
}

fn check_index(index :u32, primes :&mut HashMap<u32, u64>) -> u64 {
    {
        match primes.get(&index) {
            Some(p) => return p.clone(),
            None => ()
        }
    }

    let mut n :u64 = 0;
    let mut start_index :u32 = 0;
    match primes.keys().max() {
        Some(k) => {
            start_index = k.clone();
            n = primes.get(&k).expect("Impossible error!").clone();
        },
        None => ()
    }
    
    for current_index in start_index..index + 1 {
        loop {
            n += 1;
            if is_prime(n) {
                break;
            }
        }
        primes.insert(current_index, n);
    }

    n
}

fn is_prime(i :u64) -> bool {
    if i == 0 || i == 1 {
        return false;
    }

    let n = i / 2 + 1;
    for x in 2..n {
        if i % x == 0 {
            return false;
        }
    }

    true
}