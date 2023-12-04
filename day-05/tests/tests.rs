mod tests {
    use day_05::day_05::solve;

    #[test]
    fn test_example_01() {
        let x = solve("example.txt");
        assert!(x.is_ok());
        assert_eq!(("CMZ".to_string(), "MCD".to_string()), x.unwrap());
    }
}
