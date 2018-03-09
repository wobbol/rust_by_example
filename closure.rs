fn main()
{
    let color = "brown";

    let print = || println!("color: {}", color);

    print(); // a reference to the outer var `color` exists inside this function
    print();

    let mut count = 0;

    let mut inc = || {
        count += 1;
        println!("count: {}", count);
    };
    inc();
    inc();
    inc();
    inc();
    //let re = &mut count;



}
