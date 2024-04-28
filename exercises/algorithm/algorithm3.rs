/*
	sort
	This problem requires you to implement a sorting algorithm
	you can use bubble sorting, insertion sorting, heap sorting, etc.
*/


use std::mem::swap;

fn sort<T: Ord + std::fmt::Display>(array: &mut [T]){
	//TODO
    match array {
        [] | [_] => {},
        [a, b] => {
            if b < a {
                swap(a, b);
            }
        },
        slice => { // more than 2 elements
            let pivot = slice.len() - 1;
            let mut t = 0;
            for i in 0..slice.len() {
                if slice[pivot] > slice[i] {
                    slice.swap(i, t);
                    t = t + 1;
                }
            }
            slice.swap(pivot, t);
            sort(&mut slice[..pivot]);
            sort(&mut slice[(pivot + 1)..]);
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sort_1() {
        let mut vec = vec![37, 73, 57, 75, 91, 19, 46, 64];
        sort(&mut vec);
        assert_eq!(vec, vec![19, 37, 46, 57, 64, 73, 75, 91]);
    }
	#[test]
    fn test_sort_2() {
        let mut vec = vec![1];
        sort(&mut vec);
        assert_eq!(vec, vec![1]);
    }
	#[test]
    fn test_sort_3() {
        let mut vec = vec![99, 88, 77, 66, 55, 44, 33, 22, 11];
        sort(&mut vec);
        assert_eq!(vec, vec![11, 22, 33, 44, 55, 66, 77, 88, 99]);
    }
}