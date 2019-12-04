use rayon::prelude::*;

fn main() {
    let input = 234_208..=765_869;
    let num_valid_passwords = input
        .into_par_iter()
        .filter_map(|password| {
            if is_valid_password(password) {
                Some(password)
            } else {
                None
            }
        })
        .collect::<Vec<i32>>()
        .len();

    println!("{}", num_valid_passwords);
}

fn is_valid_password(password: i32) -> bool {
    let numbers = password
        .to_string()
        .chars()
        .filter_map(|c| c.to_digit(10))
        .collect::<Vec<u32>>();

    let not_descending = (1..(numbers.len()))
        .into_par_iter()
        .all(|i| numbers[i] >= numbers[i - 1]);

    let only_doubles_of_two = numbers
        .par_iter()
        .map(|n| {
            numbers
                .par_iter()
                .filter_map(|x| if n == x { Some(1) } else { None })
                .sum::<i32>()
                == 2
        })
        .any(|t| t);

    only_doubles_of_two && not_descending
}

#[cfg(test)]
mod tests {
    use super::is_valid_password;

    #[test]
    #[ignore]
    fn test_is_valid_password1() {
        assert_eq!(is_valid_password(111_111), true);
    }

    #[test]
    #[ignore]
    fn test_is_valid_password2() {
        assert_eq!(is_valid_password(223_450), false);
    }

    #[test]
    #[ignore]
    fn test_is_valid_password3() {
        assert_eq!(is_valid_password(123_789), false);
    }

    #[test]
    fn test_is_valid_password4() {
        assert_eq!(is_valid_password(112_233), true);
    }

    #[test]
    fn test_is_valid_password5() {
        assert_eq!(is_valid_password(123_444), false);
    }

    #[test]
    fn test_is_valid_password6() {
        assert_eq!(is_valid_password(111_122), true);
    }
}
