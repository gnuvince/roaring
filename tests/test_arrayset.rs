extern crate roaring;

use roaring::arrayset::ArraySet;

#[test]
fn test_add() {
    let mut arr = ArraySet::new();
    assert_eq!(0, arr.cardinality());
    arr.add(5);
    assert_eq!(1, arr.cardinality());
    assert!(arr.contains(5));

    // no duplicates
    arr.add(5);
    assert_eq!(1, arr.cardinality());
    assert!(arr.contains(5));

    arr.add(2);
    arr.add(3);
    arr.add(7);

    assert_eq!(4, arr.cardinality());
    assert!(arr.contains(2));
    assert!(arr.contains(3));
    assert!(arr.contains(5));
    assert!(arr.contains(7));
}


#[test]
fn test_remove() {
    let mut arr = ArraySet::new();

    arr.add(5);
    assert_eq!(1, arr.cardinality());
    assert!(arr.contains(5));

    // no effect when removing missing elements
    arr.remove(3);
    assert_eq!(1, arr.cardinality());
    assert!(arr.contains(5));

    arr.remove(5);
    assert_eq!(0, arr.cardinality());
    assert!(!arr.contains(5));
}


#[test]
fn test_union() {
    let arr1 = ArraySet::from([2, 3, 5, 7]);
    let arr2 = ArraySet::from([1, 3, 5, 7, 9]);
    let union = arr1.union(&arr2);
    assert_eq!(6, union.cardinality());
    assert!(union.contains(1));
    assert!(union.contains(2));
    assert!(union.contains(3));
    assert!(union.contains(5));
    assert!(union.contains(7));
    assert!(union.contains(9));
}


#[test]
fn test_intersection() {
    let arr1 = ArraySet::from([2, 3, 5, 7]);
    let arr2 = ArraySet::from([1, 3, 5, 7, 9]);
    let inter = arr1.intersection(&arr2);
    assert_eq!(3, inter.cardinality());
    assert!(inter.contains(3));
    assert!(inter.contains(5));
    assert!(inter.contains(7));
}

#[test]
fn test_complement() {
    let arr = ArraySet::from([2, 3, 5, 7]);
    let compl = arr.complement();
    assert_eq!(65536 - 4, compl.cardinality());
    assert!(!compl.contains(2));
    assert!(!compl.contains(3));
    assert!(!compl.contains(5));
    assert!(!compl.contains(7));
}


#[test]
fn test_difference() {
    let arr1 = ArraySet::from([2, 3, 5, 7]);
    let arr2 = ArraySet::from([1, 3, 5]);
    let diff1 = arr1.difference(&arr2);
    let diff2 = arr2.difference(&arr1);

    assert_eq!(2, diff1.cardinality());
    assert!(diff1.contains(2));
    assert!(!diff1.contains(3));
    assert!(!diff1.contains(5));
    assert!(diff1.contains(7));

    assert_eq!(1, diff2.cardinality());
    assert!(diff2.contains(1));
    assert!(!diff2.contains(3));
    assert!(!diff2.contains(5));
}


#[test]
fn test_iter() {
    let arr1 = ArraySet::from([2, 3, 5, 7]);
    let mut i = arr1.into_iter();
    assert_eq!(Some(2), i.next());
    assert_eq!(Some(3), i.next());
    assert_eq!(Some(5), i.next());
    assert_eq!(Some(7), i.next());
    assert_eq!(None, i.next());
    assert_eq!(None, i.next());
}
