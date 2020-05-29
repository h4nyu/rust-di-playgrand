trait A {
    fn a(&self) -> String;
}

trait B {
    fn b(&self) -> String;
}

struct ImplB<T>
where
    T: A,
{
    srv_a: T,
}

struct ImplA {}

impl A for ImplA {
    fn a(&self) -> String {
        format!("a")
    }
}

impl<T> B for ImplB<T>
where
    T: A,
{
    fn b(&self) -> String {
        format!("{}, {}", self.srv_a.a(), "b")
    }
}

fn main() {
    let a = ImplA {};
    let b = ImplB { srv_a: a };
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
        let b = ImplB { srv_a: MockA {} };
        assert_eq!("mock-a, b", &b.b())
    }
}
