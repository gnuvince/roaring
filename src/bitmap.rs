use std::cmp;

const DEFAULT_CAPACITY: usize = 4;
const BITS_PER_BUCKET: usize = 64;

#[derive(Debug)]
pub struct Bitmap {
    buckets: Vec<usize>,
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
        let pos = n % BITS_PER_BUCKET;
        self.buckets[bucket] |= 1 << pos;
    }


    /// Returns whether the integer `n` is in the bitmap.
    pub fn get(&self, n: usize) -> bool {
        let bucket = n / BITS_PER_BUCKET;
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


    // private
    fn num_buckets(&self) -> usize {
        return self.buckets.len();
    }
}
