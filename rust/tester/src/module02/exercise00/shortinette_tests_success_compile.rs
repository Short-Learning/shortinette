#[cfg(test)]
mod shortinette_tests_0200s {
    use ex00::{ComplexStruct, NestedStruct};

    #[test]
    fn free_complex_struct() {
        let bruh: ComplexStruct = ComplexStruct {
            name: "hey".to_string(),
            optional_value: Some(Box::new(42)),
            values: vec![1377; 5],
            some_other: vec![137700000000; 5],
            metadata: std::collections::HashMap::new(),
            nested: Box::new(NestedStruct {
                number: Box::new(42),
                optional_floats: vec![Some(Box::new(42_f64)), None, Some(Box::new(42_f64 / 2.0))],
                data: std::collections::HashMap::new(),
            }),
        };

        assert_eq!("hey", bruh.name);
    }
}
