
mod shapes {

        #[derive(Debug)]
    pub struct Circle {
         radius: f32,
    }

    impl Circle {
        pub fn new (radius: f32) -> Self {
            println!("Congratulations! Circle is created");
            Circle { radius }
        }

        pub fn new_1(radius: f32) -> Result<Circle, String> {
          if radius >= 0.0 {
            Ok(Circle { radius  })
          } else {
            Err(String::from("Radius cannot be negative"))
          }
        }

        pub fn new_2 (radius: f32) -> Circle {
           match radius {
               -10.0..=0.0 => panic!("is between -10.0 and 0.0"),
               ..=-10.0 => panic!("is lesser than -10.0"),
                _ => Circle { radius },
           }

        }
        pub fn contains (&self, other: &Circle) -> bool {
            self.radius >= other.radius
        }
    }
}


#[cfg(test)]

mod tests {
    use super::*;


    #[test]

    fn larger_circle_contains_smaller_circle() {
        let large_circle = shapes::Circle::new(10.0);
        let small_circle = shapes::Circle::new(5.0);
        assert!(large_circle.contains(&small_circle), " The larger contains the smaller circle{}", true);

        assert_ne!(large_circle.contains(&small_circle), false, " The larger contains the smaller circle");
        assert!(large_circle.contains(&small_circle));
    }


    #[test]
    fn smaller_circle_does_not_contain_larger_circle() {
        let large_circle = shapes::Circle::new(10.0);
        let small_circle = shapes::Circle::new(5.0);
        assert!(!small_circle.contains(&large_circle), " The smaller does not contain the larger circle{}", true);
    }

     #[test]
    fn should_not_create_circle_with_negative_radius() {
        let result = shapes::Circle::new_1(-5.0);
        assert!(result.is_err(), "Expected an error for negative radius");
        assert_eq!(result.unwrap_err(), "Radius cannot be negative");
    }

    #[test]
    fn should_create_circle_with_positive_radius() -> Result<(), String> {
        let some_circle = shapes::Circle::new_1(5.0)?;
        Ok(())

    }
      #[test]
    fn should_not_create_circle_with_non_positive_radius() -> Result<(), String> {
        let some_circle = shapes::Circle::new_1(5.0)?;
        Ok(())

    }

    #[test]
    #[should_panic(expected = "lesser than -10.0")]
    fn should_not_create_and_panic() {
        let some_circle = shapes::Circle::new_2(-11.0);
    }

    #[test]
    #[ignore]
    fn huge_test() {
        // This test is ignored by default
        for i in 1..1000000 {
            let large_circle = shapes::Circle::new(i as f32);
            let small_circle = shapes::Circle::new((i - 1) as f32);
            assert!(large_circle.contains(&small_circle), " The larger contains the smaller circle{}", true);
        }
    }
}

