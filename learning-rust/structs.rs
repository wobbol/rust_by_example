// Structs only contain attributes
// Traits only contain methods
//
// impls connect these two ideas

use std::fmt;

struct Color {
    red: u8,
    green: u8,
    blue: u8
}
#[derive(Debug)]
struct Kilometers(i32);

struct Example;

trait Test {
    fn foo(&self);
    fn bar(&self) {
        println!("notimplimented");
    }
    fn baz(&mut self) {
        println!("notimplimented");
    }
}

impl fmt::Display for Color {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "rgb( {}, {}, {} )", self.red, self.green, self.blue)
    }
}
impl fmt::Display for Kilometers {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} km", self.0)
    }
}
impl Test for Example {
    fn foo(&self) {
        println!("made it");
    }
}

fn main()
{
    let black = Color { red: 0,green: 0,blue: 0};
    println!("black {}", black);
    let mut link_color = Color {red: 0, green: 0, blue: 200};
    link_color.blue = 150;
    println!("link_color {}", link_color);
    println!("{}", black);
    println!("{}", black);

    let distance = Kilometers(59); //newtype pattern?
    let Kilometers(distance_in_km) = distance; // throws away the Kilometers struct
    println!("{:?}", distance);
    println!("{:?}", distance_in_km);
    Example.foo();

}
