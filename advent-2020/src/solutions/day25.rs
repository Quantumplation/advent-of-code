pub mod part1 {
    use anyhow::Result;

    use super::*;
    pub fn solve(numbers: Vec<u64>) -> Result<u64> {
        Ok(find_encryption_key(numbers[0], numbers[1]))
    }
}

pub mod part2 {
    use anyhow::Result;

    use super::*;
    pub fn solve(numbers: Vec<u64>) -> Result<String> {
        Ok("Merry Christmas!".to_string())
    }
}

pub fn find_loop_number(subject: u64, target: u64) -> u64 {
    let mut val = 1;
    let mut idx = 0;
    while val != target {
        idx += 1;
        val = (val * subject) % 20201227;
    }
    return idx;
}

pub fn find_encryption_key(door: u64, card: u64) -> u64 {
    let card_loop = find_loop_number(7, card);
    let mut val = 1;
    for _ in 0..card_loop {
        val = (val * door) % 20201227;
    }
    return val;
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn example() {
        assert_eq!(8, find_loop_number(7, 5764801));
        assert_eq!(11, find_loop_number(7, 17807724));
        assert_eq!(14897079, find_encryption_key(5764801, 17807724));
    }

    #[test]
    fn test() {
        assert_eq!(181800, find_encryption_key(8252394, 6269621));
    }
}