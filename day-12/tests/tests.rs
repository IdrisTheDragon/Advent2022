mod tests {
    use day_12::day_12::solve;

    #[test]
    fn test_example_01() {
        let x = solve("example.txt");
        assert!(x.is_ok());
        assert_eq!((31, 29), x.unwrap());
    }
}
