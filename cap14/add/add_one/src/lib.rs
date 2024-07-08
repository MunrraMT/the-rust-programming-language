use rand;

pub fn add(left: i32, right: i32) -> i32 {
    left + right
}

pub fn add_one(x: i32) -> i32 {
    x + 1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn add_its_work() {
        assert_eq!(4, add(2, 2));
    }

    #[test]
    fn add_one_its_work() {
        assert_eq!(3, add_one(2));
    }
}
