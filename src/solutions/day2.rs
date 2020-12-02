use std::str::FromStr;
use anyhow::*;

#[derive(Debug)]
pub struct PasswordRecord {
    min: u64,
    max: u64,
    char: char,
    password: String,
}

impl FromStr for PasswordRecord {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.split(" ");
        let (range, char, password) = (parts.next().ok_or(())?, parts.next().ok_or(())?, parts.next().ok_or(())?);

        let mut parts = range.split('-');
        let (min, max) = (parts.next().ok_or(())?, parts.next().ok_or(())?);
        let (min, max) = (min.parse().or(Err(()))?, max.parse().or(Err(()))?);

        let char = char.chars().next().ok_or(())?;

        let password = password.to_string();

        Ok(PasswordRecord { min, max, char, password })
    }
}

pub fn solve(inputs: Vec<PasswordRecord>) -> Result<u64> {
    count_valid(&inputs[..])
}

pub fn count_valid(inputs: &[PasswordRecord]) -> Result<u64> {
    let mut valid = 0;
    for input in inputs {
        if is_valid(&input) {
            valid += 1;
        }
    }
    Ok(valid)
}

pub fn is_valid(record: &PasswordRecord) -> bool {
    let mut count = 0;
    for c in record.password.chars() {
        if c == record.char {
            count += 1;
        }
        if count > record.max {
            return false;
        }
    }
    return count >= record.min;
}


#[cfg(test)]
mod test {
    use matches::assert_matches;

    use super::*;

    #[test]
    fn parse_success() {
        assert_matches!(
            "1-3 a: abcde".parse(),
            Ok(PasswordRecord { min: 1, max: 3, char: 'a', password: _ })
        );
        assert_eq!(
            "1-3 a: abcde".parse::<PasswordRecord>().unwrap().password,
            "abcde"
        );
    }

    #[test]
    fn valid_success() {
        assert!(is_valid(&"1-3 a: abcde".parse().unwrap()));
        assert!(is_valid(&"2-9 c: ccccccccc".parse().unwrap()));
        assert!(is_valid(&"0-0 x: abcde".parse().unwrap()));
    }

    #[test]
    fn valid_failure() {
        assert_eq!(false, is_valid(&"1-3 b: cdefg".parse().unwrap()));
        assert_eq!(false, is_valid(&"5-3 b: bbbb".parse().unwrap()));
        assert_eq!(false, is_valid(&"0-0 b: bb".parse().unwrap()));
    }

    #[test]
    fn solve_success() {
        assert_matches!(count_valid(&[
            "1-3 a: abcde".parse().unwrap(),
            "1-3 b: cdefg".parse().unwrap(),
            "2-9 c: ccccccccc".parse().unwrap(),
        ]), Ok(2));
    }
}