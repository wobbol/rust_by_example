#[derive(Debug)]
struct Structure(i32);

#[derive(Debug)]
struct Deep(Structure);

#[derive(Debug)]
struct Person<'a> {
    name: &'a str,
    age: u8
}

fn main()
{
    println!("{:?} months in a year.", 12);
    println!("{1:?} {0:?} is the {actor:?} name.",
             "Slater",
             "Christan",
             actor="actor's");

    println!("Debug printing is p-cool `{:?}`", Structure(3));
    println!("Debug printing is p-cool `{:#?}`", Deep(Structure(3)));


    let name = "Peter";
    let age = 27;
    let peter = Person{name,age};
    println!("{:#?}",peter);

}
