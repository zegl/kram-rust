use libraries::Class;

pub struct IO;

impl IO {
    fn println(&self) -> Box<Class> {
        println!("{:?}", "Look, I'm printing!");
        Box::new(IO)
    }
}

impl Class for IO {
    fn init() -> Box<Class> {
       Box::new(IO)
    }
    
    fn call(&self, method: &str) -> Box<Class> {
        let res = match method {
            "println" => self.println(),
            _ => panic!("Unknown method, {:?}", method),
        };

        Box::new(res)
    }
}