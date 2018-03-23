fn main()
{
    let mut a = Vec::new();
    let mut _b: Vec<u8> = vec![];

    for hmm in 0..20 {
    a.push(hmm);
    }
    println!("{:?}", a);

    for i in &a {
        println!("reference {}", i);
    }
    for i in &mut a {
        println!("mut reference {}", i);
    }
    for i in a {
        println!("take ownership {}", i);
    }

    // a is no longer avalable here
}
