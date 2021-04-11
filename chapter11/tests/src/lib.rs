// see lib.test.rs for tests of this function
pub fn add_two(a: i32) -> i32 {
    a + 2
}

fn sqrt(number: f64) -> Result<f64, String> {
    if number >= 0.0 {
        Ok(number.powf(0.5))
    } else {
        Err("negative floats don't have square roots".to_owned())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sqrt() -> Result<(), String> {
        let x = 4.0;
        assert_eq!(sqrt(x)?.powf(2.0), x);
        Ok(())
    }

    #[test]
    fn test_sqrt_fails_for_negative_float() {
        let x = -2.0;
        assert!(sqrt(x).is_err())
    }

    #[test]
    fn test_sqrt_fails_for_negative_float_with_result() -> Result<(), String> {
        let x = -2.0;
        match sqrt(x) {
            Ok(_) => Err("sqrt didn't fail for negative float".to_owned()),
            Err(_) => Ok(())
        }
    }
}
