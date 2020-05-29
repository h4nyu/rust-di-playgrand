trait A {
    fn a(&self);
}

trait B {
    fn b(&self);
}

struct ImplB <T>
where T: A
{
    srv_a: T
}

struct ImplA{}

impl A for ImplA{
    fn a(&self) {
        println!("a");
    }
}

impl <T> B for ImplB <T>
where T: A
{
    fn b(&self) {
        self.srv_a.a();
        println!("b");
    }
}

fn main() {
    struct MockA;
    impl A for MockA{
        fn a(&self){
            println!("mock-a");
        }
    }
    let b = ImplB {
        srv_a: MockA{}
    };
    b.b();
}
