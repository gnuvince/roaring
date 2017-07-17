extern crate roaring;

use roaring::bitmap::Bitmap;

fn main() {
    let mut primes = Bitmap::new();
    primes.set(2);
    primes.set(3);
    primes.set(5);
    primes.set(7);

    let mut odds = Bitmap::new();
    odds.set(1);
    odds.set(3);
    odds.set(5);
    odds.set(7);
    odds.set(9);

    println!("Union");
    let union = primes.union(&odds);
    for i in 0 .. 10 {
        println!("{}: {}", i, union.get(i));
    }

    println!("Intersection");
    let inter = primes.intersection(&odds);
    for i in 0 .. 10 {
        println!("{}: {}", i, inter.get(i));
    }

    println!("Out of range");
    let mut oor = Bitmap::with_capacity(1);
    let n = 64;
    println!("{}: {}", n, oor.get(n));
    oor.set(n);
    println!("{}: {}", n, oor.get(n));
}
