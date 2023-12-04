mod tests {
    use day_06::day_06::solve;

    #[test]
    fn test_example_01() {
        let x = solve("example.txt");
        assert!(x.is_ok());
        assert_eq!((7, 19), x.unwrap());
    }

    #[test]
    fn test_example_02() {
        let x = solve("example2.txt");
        assert!(x.is_ok());
        assert_eq!((5, 23), x.unwrap());
    }

    #[test]
    fn test_example_03() {
        let x = solve("example3.txt");
        assert!(x.is_ok());
        assert_eq!((6, 23), x.unwrap());
    }

    #[test]
    fn test_example_04() {
        let x = solve("example4.txt");
        assert!(x.is_ok());
        assert_eq!((10, 29), x.unwrap());
    }

    #[test]
    fn test_example_05() {
        let x = solve("example5.txt");
        assert!(x.is_ok());
        assert_eq!((11, 26), x.unwrap());
    }
}
