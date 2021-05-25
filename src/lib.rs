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
        assert_eq!(fizz_buzz(15), "FizzBuzz");
    }

}

pub fn fizz_buzz(number: u8) -> String {
    if is_fizz(number) && is_buzz(number) {
        return "FizzBuzz".to_string();
    }
    if is_fizz(number) {
        return "fizz".to_string();
    }
    if is_buzz(number) {
        return "buzz".to_string();
    }
    return number.to_string();
}

fn is_fizz(number: u8) -> bool {
    return number % 3 == 0;
}

fn is_buzz(number: u8) -> bool {
    return number % 5 == 0;
}