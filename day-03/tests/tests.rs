mod tests {
    use day_03::day_03::solve;

    #[test]
    fn test_example_01() {
        let x = solve("example.txt");
        assert!(x.is_ok());
        assert_eq!((157, 70), x.unwrap());
    }
}
