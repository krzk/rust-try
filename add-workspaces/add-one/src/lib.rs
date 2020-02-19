pub fn add_one(x: i32) -> i32 {
    x + 1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn add_one_i32() {
        assert_eq!(add_one(1), 2);
        assert_eq!(add_one(-1), 0);
    }
}
