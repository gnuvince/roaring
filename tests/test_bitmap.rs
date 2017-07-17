extern crate roaring;

use roaring::bitmap::Bitmap;


#[test]
fn test_count() {
    let mut bm = Bitmap::with_capacity(1);
    assert_eq!(0, bm.count());
    bm.set(0);
    bm.set(64);
    assert_eq!(2, bm.count());
    bm.unset(0);
    assert_eq!(1, bm.count());
    bm.unset(64);
    assert_eq!(0, bm.count());
}


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
    let primes = Bitmap::from(&[2, 3, 5, 7][..]);
    let odds = Bitmap::from(&[1, 3, 5, 7, 9][..]);
    let u = primes.union(&odds);
    assert_eq!(6, u.count());
    assert!(u.get(1));
    assert!(u.get(2));
    assert!(u.get(3));
    assert!(u.get(5));
    assert!(u.get(7));
    assert!(u.get(9));
}


#[test]
fn test_intersection() {
    let primes = Bitmap::from(&[2, 3, 5, 7][..]);
    let odds = Bitmap::from(&[1, 3, 5, 7, 9][..]);
    let u = primes.intersection(&odds);
    assert_eq!(3, u.count());
    assert!(!u.get(1));
    assert!(!u.get(2));
    assert!(u.get(3));
    assert!(u.get(5));
    assert!(u.get(7));
    assert!(!u.get(9));
}
