const CAPACITY: usize = 4096;

pub struct ArraySet {
    elements: Vec<u16>
}


impl ArraySet {
    pub fn new() -> ArraySet {
        return ArraySet {
            elements: Vec::with_capacity(CAPACITY)
        };
    }


    pub fn add(&mut self, x: u16) {
        let res = self.elements.binary_search(&x);
        match res {
            Ok(_) => (),
            Err(offset) => self.elements.insert(offset, x)
        }
    }


    pub fn remove(&mut self, x: u16) {
        let res = self.elements.binary_search(&x);
        match res {
            Err(_) => (),
            Ok(offset) => { self.elements.remove(offset); }
        }
    }


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

        for x in &self.elements[i .. ] {
            union.elements.push(*x);
        }


        for y in &other.elements[j .. ] {
            union.elements.push(*y);
        }

        return union;
    }


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


    pub fn contains(&self, x: u16) -> bool {
        return self.elements.binary_search(&x).is_ok();
    }


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


impl IntoIterator for ArraySet {
    type Item = u16;
    type IntoIter = ArraySetIter;
    fn into_iter(self) -> Self::IntoIter {
        return ArraySetIter {
            arr: self,
            offset: 0
        };
    }
}


pub struct ArraySetIter {
    arr: ArraySet,
    offset: usize,
}


impl Iterator for ArraySetIter {
    type Item = u16;

    fn next(&mut self) -> Option<Self::Item> {
        if self.offset >= self.arr.elements.len() {
            return None;
        }
        self.offset += 1;
        return Some(self.arr.elements[self.offset - 1]);
    }
}