
mod tests {
    use day_02::day_02::solve;

    #[test]
    fn test_example_01() {
        let x = solve("example.txt");
        assert!(x.is_ok());
        assert_eq!((15,12),x.unwrap());
    }
}