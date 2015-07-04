struct Number {
    value: f64,
}

impl Number {
    fn sqrt(&self) -> Number {
        Number {
            value: self.value.sqrt()
        }
    }
}

impl Class for Number {
    fn init() -> Number {
        Number{
            value: 1000.0
        }
    }
    
    fn call(&self, method: &str) -> Number {
        match method {
            "sqrt" => self.sqrt(),
            _ => panic!("Unknown method, {:?}", method),
        }
    }
}