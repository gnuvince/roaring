extern crate roaring;

use roaring::bitmap::Bitmap;


#[test]
fn test_set_unset_get() {
    let mut bm = Bitmap::with_capacity(1);
    assert!(!bm.get(0));
    bm.set(0);
    assert!(bm.get(0));
    bm.unset(0);
    assert!(!bm.get(0));
}


#[test]
fn test_union() {
    let primes = Bitmap::from(&[2, 3, 5, 7]);
    let odds = Bitmap::from(&[1, 3, 5, 7, 9]);
    let u = primes.union(&odds);
    assert_eq!(6, u.cardinality());
    assert!(u.get(1));
    assert!(u.get(2));
    assert!(u.get(3));
    assert!(u.get(5));
    assert!(u.get(7));
    assert!(u.get(9));
}


#[test]
fn test_intersection() {
    let primes = Bitmap::from(&[2, 3, 5, 7]);
    let odds = Bitmap::from(&[1, 3, 5, 7, 9]);
    let u = primes.intersection(&odds);
    assert_eq!(3, u.cardinality());
    assert!(!u.get(1));
    assert!(!u.get(2));
    assert!(u.get(3));
    assert!(u.get(5));
    assert!(u.get(7));
    assert!(!u.get(9));
}


#[test]
fn test_difference() {
    let primes = Bitmap::from(&[2, 3, 5, 7]);
    let odds = Bitmap::from(&[1, 3, 5, 7, 9]);
    let diff1 = primes.difference(&odds);
    assert_eq!(1, diff1.cardinality());
    assert!(diff1.get(2));
    assert!(!diff1.get(3));
    assert!(!diff1.get(5));
    assert!(!diff1.get(7));

    let diff2 = odds.difference(&primes);
    assert_eq!(2, diff2.cardinality());
    assert!(diff2.get(1));
    assert!(!diff2.get(2));
    assert!(!diff2.get(3));
    assert!(!diff2.get(5));
    assert!(!diff2.get(7));
    assert!(diff2.get(9));
}


#[test]
fn test_complement() {
    let primes = vec![
        2, 3, 5, 7, 11, 13, 17, 19, 23, 29,
        31, 37, 41, 43, 47, 53, 57, 59, 61
    ];
    let primes_bm = Bitmap::from(&primes);
    let composites_bm = primes_bm.complement();
    for p in primes.iter() {
        assert!(!composites_bm.get(*p));
    }
}


#[test]
fn test_iter_empty() {
    let empty_bm = Bitmap::new();
    let mut bm_iter = empty_bm.into_iter();
    assert_eq!(None, bm_iter.next());

    let mut empty_bm = Bitmap::new();
    empty_bm.compact();
    let mut bm_iter = empty_bm.into_iter();
    assert_eq!(None, bm_iter.next());
}


#[test]
fn test_iter() {
    let bm = Bitmap::from(&[2, 3, 5, 7, 131]);
    let mut bm_iter = bm.into_iter();
    assert_eq!(Some(2), bm_iter.next());
    assert_eq!(Some(3), bm_iter.next());
    assert_eq!(Some(5), bm_iter.next());
    assert_eq!(Some(7), bm_iter.next());
    assert_eq!(Some(131), bm_iter.next());
    assert_eq!(None, bm_iter.next());
    assert_eq!(None, bm_iter.next());
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


#[test]
fn test_cardinality_set_unset() {
    let mut bm = Bitmap::new();
    assert_eq!(0, bm.cardinality());
    bm.set(0);
    assert_eq!(1, bm.cardinality());
    bm.set(0);
    assert_eq!(1, bm.cardinality());
    bm.set(64);
    assert_eq!(2, bm.cardinality());
    bm.unset(64);
    bm.unset(0);
    assert_eq!(0, bm.cardinality());
}


#[test]
fn test_cardinality_from() {
    let bm1 = Bitmap::from([2, 3, 5, 7]);
    assert_eq!(4, bm1.cardinality());
}


#[test]
fn test_cardinality_union() {
    let bm1 = Bitmap::from([2, 3, 5, 7]);
    let bm2 = Bitmap::from([1, 3, 5, 7, 9]);
    let union1 = bm1.union(&bm2);
    let union2 = bm2.union(&bm1);
    assert_eq!(6, union1.cardinality());
    assert_eq!(6, union2.cardinality());
}


#[test]
fn test_cardinality_intersection() {
    let bm1 = Bitmap::from([2, 3, 5, 7]);
    let bm2 = Bitmap::from([1, 3, 5, 7, 9]);
    let inter1 = bm1.intersection(&bm2);
    let inter2 = bm2.intersection(&bm1);
    assert_eq!(3, inter1.cardinality());
    assert_eq!(3, inter2.cardinality());
}


#[test]
fn test_cardinality_difference() {
    let bm1 = Bitmap::from([2, 3, 5, 7]);
    let bm2 = Bitmap::from([1, 3, 5, 7, 9]);
    let diff1 = bm1.difference(&bm2);
    let diff2 = bm2.difference(&bm1);
    assert_eq!(1, diff1.cardinality());
    assert_eq!(2, diff2.cardinality());
}


#[test]
fn test_cardinality_complement() {
    let mut bm1 = Bitmap::with_capacity(1);
    bm1.set(0);
    let compl = bm1.complement();
    assert_eq!(63, compl.cardinality());
}
