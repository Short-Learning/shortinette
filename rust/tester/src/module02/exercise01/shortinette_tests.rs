#[cfg(test)]
mod shortinette_tests_0201 {
    use ex01::Point;
    use rand::Rng;
    use std::f32::consts::TAU;

    #[test]
    fn test_point_new() {
        let point = Point::new(12.0, 42.0);
        assert_eq!(point.x, 12.0);
        assert_eq!(point.y, 42.0);
    }

    #[test]
    fn test_point_zero() {
        let point = Point::zero();
        assert_eq!(point.x, 0.0);
        assert_eq!(point.y, 0.0);
    }

    #[test]
    fn test_point_distance_to_zero() {
        assert_eq!(Point::new(6.9, 42.0).distance(&Point::zero()), 42.563012111);
    }

    #[test]
    fn test_point_distance_same_point() {
        assert_eq!(
            Point::new(10.0, 10.0).distance(&Point::new(10.0, 10.0)),
            0.0
        );
    }

    #[test]
    fn test_point_distance_positive_negative() {
        assert_eq!(
            Point::new(-5.0, 5.0).distance(&Point::new(5.0, -5.0)),
            14.1421356237
        );
    }

    #[test]
    fn test_point_distance_large_values() {
        assert_eq!(
            Point::new(10000.0, 30000.0).distance(&Point::new(20000.0, 40000.0)),
            14142.135623730951
        );
    }

    #[test]
    fn test_point_distance_fractional() {
        assert_eq!(
            Point::new(0.5, 0.5).distance(&Point::new(1.5, 1.5)),
            1.41421356237
        );
    }

    fn setup_point_translate_and_assert(initial_point: Point, translation: Point, expected: Point) {
        let mut point = initial_point;
        point.translate(translation.x, translation.y);
        assert_eq!(point.x, expected.x);
        assert_eq!(point.y, expected.y);
    }

    #[test]
    fn test_point_translate_positive() {
        setup_point_translate_and_assert(
            Point::new(1.0, 1.0),
            Point::new(5.0, 3.0),
            Point::new(6.0, 4.0),
        );
    }

    #[test]
    fn test_point_translate_negative() {
        setup_point_translate_and_assert(
            Point::new(5.0, 5.0),
            Point::new(-2.0, -3.0),
            Point::new(3.0, 2.0),
        );
    }

    #[test]
    fn test_point_translate_zero() {
        setup_point_translate_and_assert(
            Point::new(2.0, 3.0),
            Point::new(0.0, 0.0),
            Point::new(2.0, 3.0),
        );
    }

    #[test]
    fn test_point_translate_to_zero() {
        setup_point_translate_and_assert(
            Point::new(3.0, 4.0),
            Point::new(-3.0, -4.0),
            Point::new(0.0, 0.0),
        );
    }

    #[test]
    fn test_point_translate_fractional() {
        setup_point_translate_and_assert(
            Point::new(1.5, 2.5),
            Point::new(0.5, 0.5),
            Point::new(2.0, 3.0),
        );
    }

    // For randomized tests it's impossible to always have the 100% correct expected answer, so allow small approximation.
    fn is_close(a: f32, b: f32) -> bool {
        (a - b).abs() < 0.001
    }

    #[test]
    fn test_distance_randomized() {
        let mut rng = rand::thread_rng();

        for _ in 0..100 {
            let expected_distance: f32 = rng.gen_range(1.0..100.0);

            let p1_x: f32 = rng.gen_range(-50.0..50.0);
            let p1_y: f32 = rng.gen_range(-50.0..50.0);
            let p1 = Point::new(p1_x, p1_y);

            let angle: f32 = rng.gen_range(0.0..TAU);
            let p2_x = p1_x + (expected_distance * angle.cos());
            let p2_y = p1_y + (expected_distance * angle.sin());
            let p2 = Point::new(p2_x, p2_y);

            let student_distance = p1.distance(&p2);

            assert!(
                is_close(student_distance, expected_distance),
                "Failed! Expected distance {:.3}, but got {:.3} for points ({:.2}, {:.2}) and ({:.2}, {:.2})",
                expected_distance,
                student_distance,
                p1.x,
                p1.y,
                p2.x,
                p2.y
            );
        }
    }

    #[test]
    fn test_translate_randomized() {
        let mut rng = rand::thread_rng();

        for _ in 0..100 {
            let expected_x: f32 = rng.gen_range(-100.0..100.0);
            let expected_y: f32 = rng.gen_range(-100.0..100.0);

            let dx: f32 = rng.gen_range(-50.0..50.0);
            let dy: f32 = rng.gen_range(-50.0..50.0);

            let start_x = expected_x - dx;
            let start_y = expected_y - dy;
            let mut point = Point::new(start_x, start_y);

            point.translate(dx, dy);

            assert!(
                is_close(point.x, expected_x) && is_close(point.y, expected_y),
                "Failed! Starting at ({:.2}, {:.2}) and translating by ({:.2}, {:.2}) should result in ({:.2}, {:.2}), but got ({:.2}, {:.2})",
                start_x,
                start_y,
                dx,
                dy,
                expected_x,
                expected_y,
                point.x,
                point.y
            );
        }
    }
}
