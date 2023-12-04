mod tests {
    use day_09::day_09::solve;

    #[test]
    fn test_example_01() {
        let x = solve("example.txt");
        assert!(x.is_ok());
        assert_eq!((13, 1), x.unwrap());
    }

    #[test]
    fn test_example_02() {
        let x = solve("example2.txt");
        assert!(x.is_ok());
        assert_eq!((88, 36), x.unwrap());
    }
}
