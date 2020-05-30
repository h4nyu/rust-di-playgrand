trait A {
    fn a(&self) -> String {
        format!("a")
    }
}

trait B: A {
    fn b(&self) -> String {
        format!("{}, {}", self.a(), "b")
    }
}

pub struct Context;
impl A for Context {}
impl B for Context {}

fn main() {
    let ctx = Context;
    ctx.b();
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_srv_b() {
        struct Mock;
        impl B for Mock {}
        impl A for Mock {
            fn a(&self) -> String {
                format!("mock-a")
            }
        }
        let b = Mock;
        assert_eq!("mock-a, b", &b.b())
    }
}
