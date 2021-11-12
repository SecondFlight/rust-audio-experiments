use std::collections::HashMap;

pub trait Port {}

pub trait Node<'a> {
    fn input_ports() -> &'a HashMap<u64, Box<dyn Port>>;
    fn output_ports() -> &'a HashMap<u64, Box<dyn Port>>;
}
