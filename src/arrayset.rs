// Per FSJ: initial capacity is equal to the number
// of u16 that fit in a cache line (on x86, one  cache
// line is 64 bytes; 64 bytes divided by 2 bytes is 32).
const INIT_CAPACITY: usize = 32;


/// A set of u16's represented using a vector.
#[derive(Debug)]
pub struct ArraySet {
    elements: Vec<u16>
}


impl ArraySet {
    /// Creates a new, empty set.
    pub fn new() -> ArraySet {
        return ArraySet {
            elements: Vec::with_capacity(INIT_CAPACITY)
        };
    }


    /// Adds the integer `x` to the set.
    pub fn add(&mut self, x: u16) {
        let res = self.elements.binary_search(&x);
        match res {
            Ok(_) => (),
            Err(offset) => self.elements.insert(offset, x)
        }
    }


    /// Removes the integer `x` from the set.
    pub fn remove(&mut self, x: u16) {
        let res = self.elements.binary_search(&x);
        match res {
            Err(_) => (),
            Ok(offset) => { self.elements.remove(offset); }
        }
    }


    /// Returns the union of two sets.
    // XXX: Can exceed `CAPACITY`.
    pub fn union(&self, other: &ArraySet) -> ArraySet {
        let mut union = ArraySet::new();
        let mut i = 0;
        let mut j = 0;
        while i < self.elements.len() && j < self.elements.len() {
            if self.elements[i] < other.elements[j] {
                union.elements.push(self.elements[i]);
                i += 1;
            } else if self.elements[i] > other.elements[j] {
                union.elements.push(other.elements[j]);
                j += 1;
            } else {
                union.elements.push(self.elements[i]);
                i += 1;
                j += 1;
            }
        }

        union.elements.extend(&self.elements[i..]);
        union.elements.extend(&other.elements[j..]);

        return union;
    }


    /// Returns the intersection of two sets.
    pub fn intersection(&self, other: &ArraySet) -> ArraySet {
        let mut inter = ArraySet::new();
        let mut i = 0;
        let mut j = 0;
        while i < self.elements.len() && j < self.elements.len() {
            if self.elements[i] < other.elements[j] {
                i += 1;
            } else if self.elements[i] > other.elements[j] {
                j += 1;
            } else {
                inter.elements.push(self.elements[i]);
                i += 1;
                j += 1;
            }
        }

        return inter;
    }


    /// Returns the complement of a set.
    // XXX: Can exceed `CAPACITY`.
    pub fn complement(&self) -> ArraySet {
        let mut compl = ArraySet::new();
        let mut j = 0;
        for i in 0 .. 65536_usize {
            if j < self.cardinality() && self.elements[j] == i as u16 {
                j += 1;
            } else {
                compl.elements.push(i as u16);
            }
        }
        return compl;
    }


    /// Returns the set difference of two sets.
    pub fn difference(&self, other: &ArraySet) -> ArraySet {
        let mut diff = ArraySet::new();
        let mut j = 0;
        for x in self.elements.iter() {
            while j < other.elements.len() && other.elements[j] < *x {
                j += 1;
            }
            if j >= other.elements.len() || *x < other.elements[j] {
                diff.elements.push(*x);
            }
        }
        return diff;
    }


    /// Checks if the integer `x` is contained in the set.
    pub fn contains(&self, x: u16) -> bool {
        return self.elements.binary_search(&x).is_ok();
    }


    /// Returns the number of elements in the set.
    pub fn cardinality(&self) -> usize {
        return self.elements.len();
    }
}


impl <T> From<T> for ArraySet
    where T: AsRef<[u16]>
{
     fn from(xs: T) -> ArraySet {
        let mut arr = ArraySet::new();
        for x in xs.as_ref() {
            arr.add(*x);
        }
        return arr;
    }
}


impl <'a> IntoIterator for &'a ArraySet {
    type Item = u16;
    type IntoIter = SharedArraySetIter<'a>;
    fn into_iter(self) -> Self::IntoIter {
        return SharedArraySetIter { arr: self, offset: 0 };
    }
}

impl IntoIterator for ArraySet {
    type Item = u16;
    type IntoIter = OwnedArraySetIter;
    fn into_iter(self) -> Self::IntoIter {
        return OwnedArraySetIter { arr: self, offset: 0 };
    }
}


pub struct SharedArraySetIter<'a> {
    arr: &'a ArraySet,
    offset: usize,
}


pub struct OwnedArraySetIter {
    arr: ArraySet,
    offset: usize,
}


impl <'a> Iterator for SharedArraySetIter<'a> {
    type Item = u16;

    fn next(&mut self) -> Option<Self::Item> {
        return iter_next(&mut self.offset, self.arr);
    }
}


impl Iterator for OwnedArraySetIter {
    type Item = u16;

    fn next(&mut self) -> Option<Self::Item> {
        return iter_next(&mut self.offset, &self.arr);
    }
}

fn iter_next(offset: &mut usize, arr: &ArraySet) -> Option<u16> {
    if *offset >= arr.elements.len() {
        return None;
    }
    *offset += 1;
    return Some(arr.elements[*offset - 1]);
}
