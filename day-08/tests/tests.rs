mod tests {
    use day_08::day_08::solve;

    #[test]
    fn test_example_01() {
        let x = solve("example.txt");
        assert!(x.is_ok());
        assert_eq!((21,8),x.unwrap());
    }
}