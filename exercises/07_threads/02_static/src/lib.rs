// TODO: Given a static slice of integers, split the slice into two halves and
//  sum each half in a separate thread.
//  Do not allocate any additional memory!
use std::thread;

pub fn sub_sum(v: Vec<i32>) -> i32 {
	let mut sum = 0;
	for i in v {
		sum += i
	}
	sum
}

pub fn sum(slice: &'static [i32]) -> i32 {
	let len = slice.len();
	let middle = len/2;
	let v1 = slice[0..middle].to_vec();
	let v2 = slice[middle..].to_vec();
	let handle1 = thread::spawn(|| {
		sub_sum(v1)
	});
	let handle2 = thread::spawn(|| {
		sub_sum(v2)
	});
	let sum1 = handle1.join().unwrap();
	let sum2 = handle2.join().unwrap();
	sum1 + sum2
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty() {
        static ARRAY: [i32; 0] = [];
        assert_eq!(sum(&ARRAY), 0);
    }

    #[test]
    fn one() {
        static ARRAY: [i32; 1] = [1];
        assert_eq!(sum(&ARRAY), 1);
    }

    #[test]
    fn five() {
        static ARRAY: [i32; 5] = [1, 2, 3, 4, 5];
        assert_eq!(sum(&ARRAY), 15);
    }

    #[test]
    fn nine() {
        static ARRAY: [i32; 9] = [1, 2, 3, 4, 5, 6, 7, 8, 9];
        assert_eq!(sum(&ARRAY), 45);
    }

    #[test]
    fn ten() {
        static ARRAY: [i32; 10] = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
        assert_eq!(sum(&ARRAY), 55);
    }
}
