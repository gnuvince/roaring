use std::cmp;
use std::convert::From;

const DEFAULT_CAPACITY: usize = 4;
const BITS_PER_BUCKET: usize = 64;

#[derive(Debug, Clone)]
pub struct Bitmap {
    buckets: Vec<u64>,
}


impl Bitmap {
    /// Creates a new bitmap.
    pub fn new() -> Bitmap {
        return Bitmap::with_capacity(DEFAULT_CAPACITY);
    }


    /// Creates a new bitmap with the given number
    /// of buckets; one bucket holds 64 integers.
    pub fn with_capacity(num_buckets: usize) -> Bitmap {
        return Bitmap { buckets: vec![0; num_buckets] };
    }


    /// Adds the integer `n` to the bitmap.
    pub fn set(&mut self, n: usize) {
        let bucket = n / BITS_PER_BUCKET;
        let num = self.num_buckets();
        if bucket >= num {
            self.buckets.extend(&vec![0; bucket - num + 1]);
        }
        let pos = n % BITS_PER_BUCKET;
        self.buckets[bucket] |= 1 << pos;
    }


    /// Removes the integer `n` from the bitmap.
    pub fn unset(&mut self, n: usize) {
        let bucket = n / BITS_PER_BUCKET;
        if bucket < self.num_buckets() {
            let pos = n % BITS_PER_BUCKET;
            self.buckets[bucket] &= !(1 << pos);
        }
    }


    /// Checks if the integer `n` is in the bitmap.
    pub fn get(&self, n: usize) -> bool {
        let bucket = n / BITS_PER_BUCKET;
        if bucket >= self.num_buckets() {
            return false;
        }
        let pos = n % BITS_PER_BUCKET;
        return 0 != self.buckets[bucket] & (1 << pos);
    }


    /// Computes and returns a new bitmap containing
    /// all the integers in `self` or in `other`.
    pub fn union(&self, other: &Bitmap) -> Bitmap {
        let n1 = self.num_buckets();
        let n2 = other.num_buckets();
        let mut union_bm = Bitmap::with_capacity(cmp::max(n1, n2));
        for (i, b) in union_bm.buckets.iter_mut().enumerate() {
            if i < n1 { *b |= self.buckets[i]; }
            if i < n2 { *b |= other.buckets[i]; }
        }
        return union_bm;
    }


    /// Computes and returns a new bitmap containing
    /// all the integers in `self` and in `other`.
    pub fn intersection(&self, other: &Bitmap) -> Bitmap {
        let n1 = self.num_buckets();
        let n2 = other.num_buckets();
        let mut inter_bm = Bitmap::with_capacity(cmp::min(n1, n2));
        for (i, b) in inter_bm.buckets.iter_mut().enumerate() {
            *b = self.buckets[i] & other.buckets[i];
        }
        return inter_bm;
    }


    /// Creates a new set where the elements from the
    /// bitmap on the right have been removed from the
    /// set on the right.
    pub fn difference(&self, other: &Bitmap) -> Bitmap {
        let n = cmp::min(self.num_buckets(), other.num_buckets());
        let mut diff_bm = self.clone();
        for (i, b) in (0 .. n).zip(diff_bm.buckets.iter_mut()) {
            *b &= !other.buckets[i];
        }
        return diff_bm;
    }


    /// Returns the number of integers in the bitmap.
    pub fn count(&self) -> usize {
        return self.buckets.iter()
            .map(|x| x.count_ones() as usize)
            .sum();
    }


    // private
    fn num_buckets(&self) -> usize {
        return self.buckets.len();
    }
}


impl <'a> From<&'a [usize]> for Bitmap {
    fn from(xs: &'a [usize]) -> Bitmap {
        let mut bm = Bitmap::new();
        for x in xs { bm.set(*x); }
        return bm;
    }
}
