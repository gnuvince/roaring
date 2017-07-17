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

    let union = primes.union(&odds);
    println!("Union ({})", union.count());
    for i in 0 .. 10 {
        println!("{}: {}", i, union.get(i));
    }

    let inter = primes.intersection(&odds);
    println!("Intersection ({})", inter.count());
    for i in 0 .. 10 {
        println!("{}: {}", i, inter.get(i));
    }

    println!("Out of range");
    let mut oor = Bitmap::with_capacity(1);
    let n = 64;
    println!("{}: {}", n, oor.get(n));
    oor.set(n);
    println!("{}: {}", n, oor.get(n));
    oor.unset(n);
    println!("{}: {}", n, oor.get(n));

    println!("From");
    let bm = Bitmap::from(&[2, 4, 6, 8][..]);
    for i in 0 .. 10 {
        println!("{}: {}", i, bm.get(i));
    }
}
