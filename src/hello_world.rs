use mockall::*;


#[automock]
pub trait MyTrait {
    fn foo(&self, x: u32) -> u32;
}


#[automock()]
pub mod doit{

    fn world() -> &'static str {
        "world"
    }


    pub fn hello_world() -> String {
        format!("Hello {}!", world())
    }
}

cfg_if::cfg_if! {
    if #[cfg(test)] {
        pub use mock_doit as gude;
    } else {
        pub use doit as gude;
    }
}

#[cfg(test)]
mod tests {
    use super::doit;

    #[test]
    fn test_helloworld() {

        assert_eq!(doit::hello_world(), "Hello world!");
    }
}
