pub struct MyClass {
    pub x: i32,
}

impl MyClass {
    pub fn new(x: i32) -> Self {
        Self { x }
    }
    pub fn get_double(&self) -> i32 {
        self.x * 2
    }
}
