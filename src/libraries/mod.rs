pub mod io;
use std;

pub trait Class {
    fn init() -> Box<Class>;
    fn call(&self, &str) -> Box<Class>;
}

/*impl std::fmt::Debug for Class {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "<Class>")
    }
}*/