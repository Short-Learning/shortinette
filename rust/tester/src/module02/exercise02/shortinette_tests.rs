#[cfg(test)]
mod shortinette_tests_0202 {
    use ex02::MyType;

    #[test]
    fn test_default() {
        MyType::default();
    }

    #[test]
    fn test_partial_eq_trait() {
        assert_eq!(
            MyType::default(),
            MyType::default(),
            "why would two default instances be different?"
        );
    }

    #[test]
    fn test_clone_trait() {
        let instance = MyType::default();
        let other_instance = instance.clone();
        assert_eq!(instance, other_instance, "the clone isn't the same :/");
    }

    #[test]
    fn test_partial_ord_trait() {
        let instance1 = MyType::default();
        let instance2 = MyType::default();
        assert!(
            (instance1 > instance2) == false,
            "why would the default be less than the default?"
        );
        assert!(
            (instance1 < instance2) == false,
            "why would the default be greater than the default?"
        );
    }

    #[test]
    fn test_debug_trait() {
        format!("{:?}", MyType::default());
    }
}
