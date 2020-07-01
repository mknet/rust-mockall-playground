#![cfg_attr(test, feature(proc_macro_hygiene))]

extern crate mockall;
extern crate cfg_if;

mod hello_world;
mod under_test;

struct My {}

impl hello_world::MyTrait for My {
    fn foo(&self, x: u32) -> u32 {
        x
    }
}

fn main() {
    println!("{}", hello_world::gude::hello_world());
    println!("{}", under_test::do_something());
    let my = My{};

    println!("{}", under_test::call_with_four(&my));
}

#[cfg(test)]
mod tests {

    #[test]
    fn test_helloworld_mock() {

        let ctx = super::hello_world::gude::hello_world_context();
        ctx.expect()
            .returning(|| String::from("Gude"));

        assert_eq!(super::under_test::do_something(), "Gude");
    }
}


