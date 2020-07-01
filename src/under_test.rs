use crate::hello_world::gude::hello_world;
use crate::hello_world::MyTrait;

pub fn do_something() -> String {
    hello_world()
}

pub fn call_with_four(x: &dyn MyTrait) -> u32 {
    x.foo(4)
}


#[cfg(test)]
mod tests {
    use super::*;
    use mockall::*;
    use crate::hello_world::MockMyTrait;

    #[test]
    fn test_call_with_four() {

        let mut mock = MockMyTrait::new();
        mock.expect_foo()
            .with(predicate::eq(4))
            .times(1)
            .returning(|x| x + 1);
        assert_eq!(5, call_with_four(&mock));
    }
}
