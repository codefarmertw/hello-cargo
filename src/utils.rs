pub fn add(a: i8, b: i8) -> i8 {
  if a > 50 || b > 50 {
      panic!("請輸入小於 50 的值！你輸入的 a 是 {}, b 是 {}", a, b);
  }
  a + b
}

pub fn sub(a: i8, b: i8) -> i8 {
    a - b
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add() {
        assert_eq!(add(2, 3), 5);
        assert_eq!(add(-1, 1), 0);
        assert_eq!(add(0, 0), 0);
        assert_eq!(add(-5, -7), -12);
    }

    #[test]
    fn test_sub() {
        assert_eq!(sub(5, 3), 2);
        assert_eq!(sub(-1, 1), -2);
        assert_eq!(sub(0, 0), 0);
        assert_eq!(sub(-5, -7), 2);
    }

    #[test]
    #[should_panic(expected = "請輸入小於 50 的值")]
    fn some_error() {
        assert_eq!(add(12, 34), 46);
        panic!("不是程式碼中的 panic");
    }
}
