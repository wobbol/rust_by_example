
fn call_me<F: Fn()>(f: F) -> F
{
    f
}

fn what()
{
    println!("big thonk");
}

fn main()
{

    println!("hmm");
    let grape = call_me(what);
    println!("hmm");
    println!("hmm {:?}",grape());

    fn a_fun(i: i32) -> i32 { i + 1 }
    let c_annotated = |i: i32| -> i32 { i + 1 };
    let c_inferred = |i| i + 1;

    let i = 1;

    println!("fun {}",a_fun(i));
    println!("annotated {}",c_annotated(i));
    println!("inferred {}",c_inferred(i));


}
