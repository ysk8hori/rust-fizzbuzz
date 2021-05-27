#[cfg(test)]
mod fizz_buzz {
    use super::*;

    #[test]
    fn _2_is_2() {
        assert_eq!(fizz_buzz(2), "2");
    }

    #[test]
    fn _3_is_fizz() {
        assert_eq!(fizz_buzz(3), "fizz");
    }

    #[test]
    fn _5_is_buzz() {
        assert_eq!(fizz_buzz(5), "buzz");
    }

    #[test]
    fn _15_is_fizzbuzz() {
        assert_eq!(fizz_buzz(15), "fizzbuzz");
    }

    #[test]
    fn count_fizz_buzz_1to30() {
        assert_eq!(count_fizz_buzz(1, 30), 1);
    }

    #[test]
    fn count_fizz_buzz_1to31() {
        assert_eq!(count_fizz_buzz(1, 31), 2);
    }

    #[test]
    fn count_fizz_buzz_1to101() {
        assert_eq!(count_fizz_buzz(1, 101), 6);
    }
}

pub fn fizz_buzz(number: u8) -> String {
    if is_fizz(number) && is_buzz(number) {
        "fizzbuzz".to_string()
    } else if is_fizz(number) {
        "fizz".to_string()
    } else if is_buzz(number) {
        "buzz".to_string()
    } else {
        number.to_string()
    }
}

fn is_fizz(number: u8) -> bool {
    number % 3 == 0
}

fn is_buzz(number: u8) -> bool {
    number % 5 == 0
}

pub fn count_fizz_buzz(start: u8, end: u8) -> u8 {
    let mut number = start;
    let mut fizz_buzz_count = 0;
    return loop {
        if fizz_buzz(number) == "fizzbuzz" {
            fizz_buzz_count += 1;
        }
        number += 1;
        if end <= number {
            break fizz_buzz_count;
        }
    }
}
