extern crate roaring;

use std::io;
use std::time::Instant;

use roaring::bitmap::Bitmap;

fn main() {
    let t = Instant::now();

    let mut bm = Bitmap::new();
    let mut buf = String::new();
    let stdin = io::stdin();
    while stdin.read_line(&mut buf).unwrap() > 0 {
        let n = buf.trim().parse::<i32>().unwrap();
        bm.set(n);
        buf.clear();
    }
    println!("{} entries", bm.count());
    println!("{} buckets", bm.num_buckets());
    println!("{:?}", t.elapsed());
}
