use crate::core::object_model::attr_model;
use crate::core::object_model::object_model;

pub fn run() {
    attr_model::AttrModel::new(12);
    let oo = object_model::ObjectModel::new(String::from("foo"));
    oo.print();
    println!("1: {}", oo.get_attr_value(1));
    println!("2: {}", oo.get_attr_value(2));
    println!("3: {}", oo.get_attr_value(3));
}
