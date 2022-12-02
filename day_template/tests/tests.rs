mod tests {
    use {{project-name | snake_case}}::{{project-name | snake_case}}::solve;

    #[test]
    fn test_example_01() {
        let x = solve("example.txt");
        assert!(x.is_ok());
        assert_eq!((0,0),x.unwrap());
    }
}