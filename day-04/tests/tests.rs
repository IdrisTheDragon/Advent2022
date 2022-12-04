mod tests {
    use day_04::day_04::solve;

    #[test]
    fn test_example_01() {
        let x = solve("example.txt");
        assert!(x.is_ok());
        assert_eq!((2,4),x.unwrap());
    }
}