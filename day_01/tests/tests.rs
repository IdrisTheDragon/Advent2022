
mod tests {
    use day_01::day_01::solve;

    #[test]
    fn test_example_01() {
        let x = solve("example.txt");
        assert!(x.is_ok());
        assert_eq!((24000,45000),x.unwrap());
    }
}