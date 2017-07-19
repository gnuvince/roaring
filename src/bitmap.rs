use std::cmp;
use std::convert::{AsRef, From};


const DEFAULT_CAPACITY: usize = 4;
const BITS_PER_BUCKET: usize = 64;


/// A bitmap implementation.
///
/// An efficient set implementation making use of
/// bit-level parallelism to improve the performance
/// of aggregate operations. It supports the following
/// operations:
///
/// - Adding/removing an integer;
/// - Querying for membership;
/// - Performing the union, intersection, difference, and complement;
/// - Querying the number of members.
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
    pub fn set(&mut self, n: i32) {
        let bucket = n as usize / BITS_PER_BUCKET;
        let num = self.num_buckets();
        if bucket >= num {
            self.buckets.extend(&vec![0; bucket - num + 1]);
        }
        let pos = n as usize % BITS_PER_BUCKET;
        self.buckets[bucket] |= 1 << pos;
    }


    /// Removes the integer `n` from the bitmap.
    pub fn unset(&mut self, n: i32) {
        let bucket = n as usize / BITS_PER_BUCKET;
        if bucket < self.num_buckets() {
            let pos = n as usize % BITS_PER_BUCKET;
            self.buckets[bucket] &= !(1 << pos);
        }
    }


    /// Checks if the integer `n` is in the bitmap.
    pub fn get(&self, n: i32) -> bool {
        let bucket = n as usize / BITS_PER_BUCKET;
        if bucket >= self.num_buckets() {
            return false;
        }
        let pos = n as usize % BITS_PER_BUCKET;
        return 0 != self.buckets[bucket] & (1 << pos);
    }


    /// Returns the set union of two bitmaps.
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


    /// Returns the set intersection of two bitmaps.
    pub fn intersection(&self, other: &Bitmap) -> Bitmap {
        let n1 = self.num_buckets();
        let n2 = other.num_buckets();
        let mut inter_bm = Bitmap::with_capacity(cmp::min(n1, n2));
        for (i, b) in inter_bm.buckets.iter_mut().enumerate() {
            *b = self.buckets[i] & other.buckets[i];
        }
        return inter_bm;
    }


    /// Returns the difference between two sets;
    /// an integer `n` is included in the result
    /// if it is present in the set of the left,
    /// but not in the set on the right.
    pub fn difference(&self, other: &Bitmap) -> Bitmap {
        let n = cmp::min(self.num_buckets(), other.num_buckets());
        let mut diff_bm = self.clone();
        for (i, b) in (0 .. n).zip(diff_bm.buckets.iter_mut()) {
            *b &= !other.buckets[i];
        }
        return diff_bm;
    }


    /// Returns the complement of a bitmap.  Only the
    /// bits present in the buckets are flipped: no new
    /// integers are added to the set.
    pub fn complement(&self) -> Bitmap {
        let mut compl_bm = self.clone();
        for b in compl_bm.buckets.iter_mut() {
            *b = !*b;
        }
        return compl_bm;
    }


    /// Returns the number of integers in the bitmap.
    pub fn count(&self) -> usize {
        return self.buckets.iter()
            .map(|x| x.count_ones() as usize)
            .sum();
    }


    /// Removes the unnecessary buckets, i.e., the ones
    /// at the end of the vector that are zero. Stop at
    /// the first non-zero bucket or when all buckets
    /// have been removed.
    pub fn compact(&mut self) {
        while !self.buckets.is_empty() {
            if self.buckets[self.buckets.len() - 1] == 0 {
                self.buckets.pop();
            } else {
                break;
            }
        }
    }


    pub fn num_buckets(&self) -> usize {
        return self.buckets.len();
    }
}


impl <T> From<T> for Bitmap
    where T: AsRef<[i32]>
{
    fn from(elems: T) -> Bitmap {
        let mut bm = Bitmap::new();
        for x in elems.as_ref() {
            bm.set(*x);
        }
        return bm;
    }
}


impl IntoIterator for Bitmap {
    type Item = i32;
    type IntoIter = BitmapIterator;
    fn into_iter(self) -> Self::IntoIter {
        return BitmapIterator { offset: 0, bitmap: self };
    }
}


pub struct BitmapIterator {
    offset: usize,
    bitmap: Bitmap,
}


impl Iterator for BitmapIterator {
    type Item = i32;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            let bucket = self.offset / BITS_PER_BUCKET;
            let bit_pos = self.offset % BITS_PER_BUCKET;

            if bucket >= self.bitmap.num_buckets() {
                return None;
            }

            // Optimization: skip whole bucket when its value is zero.
            if self.bitmap.buckets[bucket] == 0 {
                self.offset += 64;
                continue;
            }

            if self.bitmap.buckets[bucket] & (1 << bit_pos) == 0 {
                self.offset += 1;
                continue;
            }

            let x = self.offset;
            self.offset += 1;
            return Some(x as i32);
        }
    }
}


#[test]
fn test_compact() {
    let mut bm = Bitmap::new();
    assert_eq!(4, bm.num_buckets());
    bm.compact();
    assert_eq!(0, bm.num_buckets());
    bm.set(0);
    bm.set(128);
    assert_eq!(3, bm.num_buckets());
    bm.compact();
    assert_eq!(3, bm.num_buckets());
    bm.unset(128);
    bm.compact();
    assert_eq!(1, bm.num_buckets());
}
