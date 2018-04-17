fn parrot(s:&str, _len: usize)
{
    println!("foo: {}", s);
}

fn echo(s:&str, len: usize)
{
    if s.len() > len {

        let (_, stuff) = s.split_at(len);
        println!("echo: {}", stuff);
    }
}

struct Action<'a> {
    func: &'a Fn(&str, usize),
    name: String,
}

fn main()
{
    let input = String::from("/echo, things, foo, and bar.");

    let mut act: Vec<Action> = Vec::new();
    act.push(Action{func:&parrot, name:String::from("/parrot")});
    act.push(Action{func:&echo, name:String::from("/echo")});

    for a in &act {
        if input.starts_with(&a.name) {
            (*a.func)(&input, a.name.len());
            return;
        }
    }
}
