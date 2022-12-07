mod tests {
    use day_07::day_07::solve;

    #[test]
    fn test_example_01() {
        let x = solve("example.txt");
        assert!(x.is_ok());
        assert_eq!((95437,24933642),x.unwrap());
    }
}