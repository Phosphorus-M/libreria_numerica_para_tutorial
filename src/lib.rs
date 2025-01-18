pub mod prelude {
    pub fn add(left: u64, right: u64) -> u64 {
        left + right + 2
    }
    
    pub struct Point {
        pub x: f64,
        pub y: f64
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
