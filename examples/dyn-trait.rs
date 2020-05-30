trait A {
    fn a(&self) -> String;
}

trait B {
    fn b(&self) -> String;
}

struct ImplA {}

impl A for ImplA {
    fn a(&self) -> String {
        format!("a")
    }
}

struct ImplB {
    srv_a: Box<dyn A>,
}

impl B for ImplB {
    fn b(&self) -> String {
        format!("{}, {}", self.srv_a.a(), "b")
    }
}

fn main() {
    let a = ImplA {};
    let b = ImplB { srv_a: Box::new(a) };
    b.b();
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_srv_b() {
        struct MockA;
        impl A for MockA {
            fn a(&self) -> String {
                format!("mock-a")
            }
        }
        let mock_a = MockA {};
        let b = ImplB {
            srv_a: Box::new(mock_a),
        };
        assert_eq!("mock-a, b", &b.b())
    }
}
