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
    cardinality: usize,
}


impl Bitmap {
    /// Creates a new bitmap.
    pub fn new() -> Bitmap {
        return Bitmap::with_capacity(DEFAULT_CAPACITY);
    }


    /// Creates a new bitmap with the given number
    /// of buckets; one bucket holds 64 integers.
    pub fn with_capacity(num_buckets: usize) -> Bitmap {
        return Bitmap {
            buckets: vec![0; num_buckets],
            cardinality: 0
        };
    }


    /// Adds the integer `n` to the bitmap.
    pub fn set(&mut self, n: u16) {
        let bucket = n as usize / BITS_PER_BUCKET;
        let num = self.num_buckets();
        if bucket >= num {
            self.buckets.extend(&vec![0; bucket - num + 1]);
        }
        let pos = n as usize % BITS_PER_BUCKET;
        // Add 1 to cardinality if bit isn't set.
        self.cardinality += 1 - ((self.buckets[bucket] >> pos) & 1) as usize;
        self.buckets[bucket] |= 1 << pos;
    }


    /// Removes the integer `n` from the bitmap.
    pub fn unset(&mut self, n: u16) {
        let bucket = n as usize / BITS_PER_BUCKET;
        if bucket < self.num_buckets() {
            let pos = n as usize % BITS_PER_BUCKET;
            // Subtract 1 from cardinality if bit is set.
            self.cardinality -= ((self.buckets[bucket] >> pos) & 1) as usize;
            self.buckets[bucket] &= !(1 << pos);
        }
    }


    /// Checks if the integer `n` is in the bitmap.
    pub fn get(&self, n: u16) -> bool {
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
        union_bm.cardinality = union_bm.count();
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
        inter_bm.cardinality = inter_bm.count();
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
        diff_bm.cardinality = diff_bm.count();
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
        compl_bm.cardinality =
            (compl_bm.num_buckets() * BITS_PER_BUCKET) - self.cardinality;
        return compl_bm;
    }


    /// Returns the number of integers in the bitmap.
    fn count(&self) -> usize {
        return self.buckets.iter()
            .map(|x| x.count_ones() as usize)
            .sum();
    }


    /// Returns the cardinality of the bitmap.
    pub fn cardinality(&self) -> usize {
        return self.cardinality;
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
    where T: AsRef<[u16]>
{
    fn from(elems: T) -> Bitmap {
        let mut bm = Bitmap::new();
        for x in elems.as_ref() {
            bm.set(*x);
        }
        return bm;
    }
}


impl <'a> IntoIterator for &'a Bitmap {
    type Item = u16;
    type IntoIter = SharedBitmapIterator<'a>;
    fn into_iter(self) -> Self::IntoIter {
        return SharedBitmapIterator { offset: 0, bitmap: &self };
    }
}

impl IntoIterator for Bitmap {
    type Item = u16;
    type IntoIter = OwnedBitmapIterator;
    fn into_iter(self) -> Self::IntoIter {
        return OwnedBitmapIterator { offset: 0, bitmap: self };
    }
}


pub struct SharedBitmapIterator<'a> {
    offset: usize,
    bitmap: &'a Bitmap,
}


pub struct OwnedBitmapIterator {
    offset: usize,
    bitmap: Bitmap,
}


impl <'a> Iterator for SharedBitmapIterator<'a> {
    type Item = u16;

    fn next(&mut self) -> Option<Self::Item> {
        return next_iter(&mut self.offset, self.bitmap);
    }
}

impl Iterator for OwnedBitmapIterator {
    type Item = u16;

    fn next(&mut self) -> Option<Self::Item> {
        return next_iter(&mut self.offset, &self.bitmap);
    }
}


fn next_iter(offset: &mut usize, bitmap: &Bitmap) -> Option<u16> {
    loop {
        let bucket = *offset / BITS_PER_BUCKET;
        let bit_pos = *offset % BITS_PER_BUCKET;

        if bucket >= bitmap.num_buckets() {
            return None;
        }

        // Optimization: skip whole bucket when its value is zero.
        if bitmap.buckets[bucket] == 0 {
            *offset += 64;
            continue;
        }

        if bitmap.buckets[bucket] & (1 << bit_pos) == 0 {
            *offset += 1;
            continue;
        }

        let x = *offset;
        *offset += 1;
        return Some(x as u16);
    }

}
