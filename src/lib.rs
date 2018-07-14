mod client;

mod network { // declare a module named network
    // declare a function connect within the module
    fn connect() {
    }

    mod server {
        fn connect() {
        }
    }
}


#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
