#[cfg(test)]
use super::SortedArray;


#[test]
fn create_empty() {
    let a = SortedArray::<u32>::new();
    assert_eq!(0, a.len());
}


#[test]
fn insert_inorder() {
    
    let mut a = SortedArray::<u32>::new();
    
    a.insert(99);
    a.insert(100);
    a.insert(1000);

    assert_eq!(3, a.len());
    assert_eq!(0, a.find(99));
    assert_eq!(1, a.find(100));
    assert_eq!(2, a.find(1000));
}


#[test]
fn insert_outoforder() {
    
    let mut a = SortedArray::<u32>::new();
    
    a.insert(99);
    a.insert(32);
    a.insert(16);
    a.insert(50);

    assert_eq!(4, a.len());
    assert_eq!(0, a.find(16));
    assert_eq!(1, a.find(32));
    assert_eq!(2, a.find(50));
    assert_eq!(3, a.find(99));
}


#[test]
fn find_missing() {
    
    let mut a = SortedArray::<u32>::new();
    
    a.insert(99);
    a.insert(32);
    a.insert(50);
    a.insert(16);

    assert_eq!(4, a.len());
    assert_eq!(0, a.find(15));
    assert_eq!(1, a.find(31));
    assert_eq!(2, a.find(49));
    assert_eq!(3, a.find(90));
    assert_eq!(4, a.find(100));
}
