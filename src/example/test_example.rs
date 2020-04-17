#[allow(dead_code)]
fn give_two() -> i32 {
    return 2;
}

#[allow(dead_code)]
struct Retangle {
    width: i8,
    height: i8,
}

#[allow(dead_code)]
impl Retangle {
    fn is_square(&self) -> bool {
        return self.height == self.width;
    }
}

#[cfg(test)]
mod test {
    #[test]
    fn test_basic() {
        assert!(1 == 1);
    }

    #[test]
    fn panic() {
        panic!("I'm panicing");
    }

    #[test]
    #[should_panic]
    fn should_panic() {
        panic!("Yes");
    }

    #[test]
    fn test_function() {
        assert_eq!(super::give_two(), 1 + 1);
    }

    #[test]
    fn test_struct() {
        let retangle = super::Retangle {
            width: 10,
            height: 5,
        };

        assert_eq!(retangle.is_square(), false);
    }
}
