pub fn add(x: i32, y: i32) -> i32 {
    return x + y;
}

pub fn sub(x: i32, y: i32) -> i32 {
    return x - y;
}

pub fn sqrt(number: f64) -> Result<f64, String> {
    if number >= 0.0 {
        Ok(number.powf(0.5))
    } else {
        Err("negative floats don't have square roots".to_owned())
    }
}

// please uncomment this for production
// #[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add() {
        assert_eq!(add(1, 2), 3);
    }

    #[test]
    fn test_sub() {
        assert_eq!(sub(10, 5), 5);

        assert!(true);
    }

    #[test]
    fn just_panic() {
        panic!("Divide-by-zero error");
    }

    #[test]
    fn test_sqrt() -> Result<(), String> {
        let x = 4.0;
        assert_eq!(sqrt(x)?.powf(2.0), x);
        assert_eq!(sqrt(-10f64)?.powf(2.0), x);
        Ok(())
    }
}

fn main() {}
