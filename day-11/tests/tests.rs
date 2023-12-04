mod tests {
    use day_11::day_11::solve;

    #[test]
    fn test_example_01() {
        let x = solve("example.txt");
        assert!(x.is_ok());
        assert_eq!((10605, 2713310158), x.unwrap());
    }
}
