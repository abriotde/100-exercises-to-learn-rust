// TODO: Given a vector of integers, leak its heap allocation.
//  Then split the resulting static slice into two halves and
//  sum each half in a separate thread.
//  Hint: check out `Vec::leak`.

use std::thread;
pub fn sub_sum(v: Vec<i32>) -> i32 {
	let mut sum = 0;
	for i in v {
		sum += i
	}
	sum
}

pub fn sum(v: Vec<i32>) -> i32 {
	let v0 = v.leak();
	let len = v0.len();
	let middle = len/2;
	let v1 = v0[0..middle].to_vec();
	let v2 = v0[middle..].to_vec();
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
        assert_eq!(sum(vec![]), 0);
    }

    #[test]
    fn one() {
        assert_eq!(sum(vec![1]), 1);
    }

    #[test]
    fn five() {
        assert_eq!(sum(vec![1, 2, 3, 4, 5]), 15);
    }

    #[test]
    fn nine() {
        assert_eq!(sum(vec![1, 2, 3, 4, 5, 6, 7, 8, 9]), 45);
    }

    #[test]
    fn ten() {
        assert_eq!(sum(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10]), 55);
    }
}
