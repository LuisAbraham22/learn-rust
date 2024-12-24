mod basket;
mod container;
mod stack;

use basket::Basket;
use container::Container;
use num_traits::ToPrimitive;
use stack::Stack;

fn solve<T, U>(a: T, b: U) -> f64
where
    T: ToPrimitive,
    U: ToPrimitive,
{
    let a_f64 = a.to_f64().unwrap();
    let b_f64 = b.to_f64().unwrap();

    (a_f64.powi(2) + b_f64.powi(2)).sqrt()
}

fn add_string<T: Container<String>>(container: &mut T, string: String) {
    container.put(string);
}

fn main() {
    // let a: i32 = 3;
    // let b: f64 = 4.0;
    // println!("{}", solve(a, b))
    //
    let mut s1 = Stack::new(vec![1, 2, 3]);
    // Trait bound prevents storing a string to the integer stack
    // add_string(&mut s1, String::from("value"));
    let mut b1 = Basket::new(String::from("Luis"));
    add_string(&mut b1, String::from("Goldie"));
}
