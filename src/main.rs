mod my_class;
use my_class::MyClass;
 

fn main() {
    for i in 1..10 {
        let a = MyClass::new(i);
        println!("{}",a.get_double());
    }
}
