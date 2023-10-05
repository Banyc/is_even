#![feature(test)]

extern crate test;

pub fn is_even(number: i8) -> bool {
    match number {
        0 => true,
        1 => false,
        _ => is_even(number.wrapping_sub(2)),
    }
}

#[cfg(test)]
mod tests {
    use test::{black_box, Bencher};

    use super::*;

    #[test]
    fn test() {
        assert!(is_even(0));
        assert!(!is_even(1));
        assert!(is_even(2));
        assert!(!is_even(3));
        assert!(is_even(4));
        assert!(!is_even(-1));
        assert!(is_even(-2));
    }

    #[bench]
    fn soy(b: &mut Bencher) {
        b.iter(|| {
            for _ in 0..=u8::MAX {
                for number in i8::MIN..=i8::MAX {
                    black_box(number % 2);
                }
            }
        });
    }

    #[bench]
    fn chad(b: &mut Bencher) {
        b.iter(|| {
            for _ in 0..=u8::MAX {
                for number in i8::MIN..=i8::MAX {
                    black_box(is_even(number));
                }
            }
        });
    }
}
