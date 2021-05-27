#[cfg(test)]
mod fizzbuzz {
    use super::*;

    #[test]
    fn _2_is_2() {
        assert_eq!(fizzbuzz(2), "2");
    }

    #[test]
    fn _3_is_fizz() {
        assert_eq!(fizzbuzz(3), "fizz");
    }

    #[test]
    fn _5_is_buzz() {
        assert_eq!(fizzbuzz(5), "buzz");
    }

    #[test]
    fn _15_is_fizzbuzz() {
        assert_eq!(fizzbuzz(15), "fizzbuzz");
    }

    #[test]
    fn count_fizzbuzz_1to30() {
        assert_eq!(count_fizzbuzz(1, 30), 1);
    }

    #[test]
    fn count_fizzbuzz_1to31() {
        assert_eq!(count_fizzbuzz(1, 31), 2);
    }

    #[test]
    fn fizzbuzz_all_() {
        let result = fizzbuzz_all(vec![0, 2,3,5,15]);
        assert_eq!(result[0], "fizzbuzz");
        assert_eq!(result[1], "2");
        assert_eq!(result[2], "fizz");
        assert_eq!(result[3], "buzz");
        assert_eq!(result[4], "fizzbuzz");
    }
}

pub fn fizzbuzz(number: u8) -> String {
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

pub fn count_fizzbuzz(start: u8, end: u8) -> u8 {
    let mut number = start;
    let mut fizzbuzz_count = 0;
    return loop {
        if fizzbuzz(number) == "fizzbuzz" {
            fizzbuzz_count += 1;
        }
        number += 1;
        if end <= number {
            break fizzbuzz_count;
        }
    }
}

pub fn fizzbuzz_all(numbers:Vec<u8>) -> Vec<String> {
    let mut result_vec = Vec::new();
    for num in numbers.iter() {
        result_vec.push(fizzbuzz(*num));
    }
    result_vec
}