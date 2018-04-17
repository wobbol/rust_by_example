struct Ref<'a, T: 'a>(&'a T); //If T contains any references, they must live longer than T
struct StaticRef<T: 'static>(&'static T); //T may not contain reference types, unless they live as long as the program.

trait Red {}

struct Ball<'a> {
    diameter: &'a i32,
}
impl<'a> Red for Ball<'a> {}

fn main()
{
    let dog = String::from("what");
    let _something: Ref<String> = Ref(&dog);

    static STATIC_DOG: i32 = 1;
    let _or_other: StaticRef<i32> = StaticRef(&STATIC_DOG);

    //TODO: understand this
    let num = 5;
    let _obj = Box::new(Ball { diameter: &num }) as Box<Red>;
}
